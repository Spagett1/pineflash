#Maintainer: Kartoffel <laar@tutanota.com>
pkgname=pineflash-git
pkgver=0.2.1
pkgrel=1
arch=('any')
pkgdesc='A rust program for flashing pinecils and in the future other pine64 products.'
url='https://github.com/Laar3/PineFlash'
license=('GPLv2')
depends=('polkit' 'dfu-util')
makedepends=('rust')
source=(PineFlash::git+https://github.com/Laar3/PineFlash#branch=main)
md5sums=('SKIP')

package() {
  cd $srcdir/PineFlash
  cargo build --release
  mkdir -p $pkgdir/usr/share/{pixmaps,applications}
  mkdir -p $pkgdir/usr/bin/
  mv $srcdir/PineFlash/target/release/pineflash $pkgdir/usr/bin/
  mv ./Pineflash.desktop $pkgdir/usr/share/applications/
  mv ./assets/pine64logo.png $pkgdir/usr/share/pixmaps/
}
