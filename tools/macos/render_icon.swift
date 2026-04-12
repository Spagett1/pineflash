import AppKit
import CoreGraphics
import Foundation

let arguments = CommandLine.arguments
guard arguments.count == 3, let size = Int(arguments[1]) else {
    FileHandle.standardError.write(Data("usage: render_icon.swift <size> <output-path>\n".utf8))
    exit(1)
}

let outputURL = URL(fileURLWithPath: arguments[2])
let projectRoot = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let sourceIconURL = projectRoot.appendingPathComponent("icon.png")

guard let sourceImage = NSImage(contentsOf: sourceIconURL),
      let sourceCGImage = sourceImage.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
    FileHandle.standardError.write(Data("failed to load \(sourceIconURL.path)\n".utf8))
    exit(1)
}

let sourceWidth = sourceCGImage.width
let sourceHeight = sourceCGImage.height
let colorSpace = CGColorSpaceCreateDeviceRGB()
let bytesPerPixel = 4
let bytesPerRow = sourceWidth * bytesPerPixel

guard let sourceContext = CGContext(
    data: nil,
    width: sourceWidth,
    height: sourceHeight,
    bitsPerComponent: 8,
    bytesPerRow: bytesPerRow,
    space: colorSpace,
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
) else {
    FileHandle.standardError.write(Data("failed to create source bitmap context\n".utf8))
    exit(1)
}

sourceContext.interpolationQuality = .high
sourceContext.draw(sourceCGImage, in: CGRect(x: 0, y: 0, width: sourceWidth, height: sourceHeight))

guard let sourceData = sourceContext.data else {
    FileHandle.standardError.write(Data("failed to access source bitmap data\n".utf8))
    exit(1)
}

let pixels = sourceData.bindMemory(to: UInt8.self, capacity: sourceWidth * sourceHeight * bytesPerPixel)

var minX = sourceWidth
var minY = sourceHeight
var maxX = 0
var maxY = 0
let whiteThreshold: UInt8 = 245

for y in 0..<sourceHeight {
    for x in 0..<sourceWidth {
        let offset = y * bytesPerRow + x * bytesPerPixel
        let red = pixels[offset]
        let green = pixels[offset + 1]
        let blue = pixels[offset + 2]
        let alpha = pixels[offset + 3]

        if alpha > 0 && red >= whiteThreshold && green >= whiteThreshold && blue >= whiteThreshold {
            pixels[offset] = 0
            pixels[offset + 1] = 0
            pixels[offset + 2] = 0
            pixels[offset + 3] = 0
            continue
        }

        if pixels[offset + 3] > 0 {
            minX = min(minX, x)
            minY = min(minY, y)
            maxX = max(maxX, x)
            maxY = max(maxY, y)
        }
    }
}

guard let cleanedImage = sourceContext.makeImage() else {
    FileHandle.standardError.write(Data("failed to create cleaned source image\n".utf8))
    exit(1)
}

let cropRect: CGRect
if minX <= maxX && minY <= maxY {
    cropRect = CGRect(
        x: minX,
        y: minY,
        width: maxX - minX + 1,
        height: maxY - minY + 1
    )
} else {
    cropRect = CGRect(x: 0, y: 0, width: sourceWidth, height: sourceHeight)
}

let croppedImage = cleanedImage.cropping(to: cropRect.integral) ?? cleanedImage

let canvas = size
let inset = Int(Double(canvas) * 0.12)
let outputWidth = canvas
let outputHeight = canvas
let availableWidth = outputWidth - inset * 2
let availableHeight = outputHeight - inset * 2

guard let outputContext = CGContext(
    data: nil,
    width: outputWidth,
    height: outputHeight,
    bitsPerComponent: 8,
    bytesPerRow: outputWidth * bytesPerPixel,
    space: colorSpace,
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
) else {
    FileHandle.standardError.write(Data("failed to create output bitmap context\n".utf8))
    exit(1)
}

outputContext.interpolationQuality = .high
let croppedWidth = croppedImage.width
let croppedHeight = croppedImage.height
let widthScale = Double(availableWidth) / Double(croppedWidth)
let heightScale = Double(availableHeight) / Double(croppedHeight)
let scale = min(widthScale, heightScale)
let drawWidth = Double(croppedWidth) * scale
let drawHeight = Double(croppedHeight) * scale
let drawRect = CGRect(
    x: (Double(outputWidth) - drawWidth) / 2.0,
    y: (Double(outputHeight) - drawHeight) / 2.0,
    width: drawWidth,
    height: drawHeight
)
outputContext.draw(croppedImage, in: drawRect)

guard let outputImage = outputContext.makeImage() else {
    FileHandle.standardError.write(Data("failed to create output image\n".utf8))
    exit(1)
}

let bitmap = NSBitmapImageRep(cgImage: outputImage)
guard let pngData = bitmap.representation(using: .png, properties: [:]) else {
    FileHandle.standardError.write(Data("failed to encode png\n".utf8))
    exit(1)
}

try FileManager.default.createDirectory(
    at: outputURL.deletingLastPathComponent(),
    withIntermediateDirectories: true
)
try pngData.write(to: outputURL)
