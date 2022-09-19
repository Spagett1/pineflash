curl -s https://api.github.com/repos/Ralim/IronOS/releases/latest | \
  grep browser_download_url | \
  cut -d : -f2,3 | \
  sed 's/^ [ \t]*//' 
