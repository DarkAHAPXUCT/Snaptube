#!/data/data/com.termux/files/usr/bin/bash
cd ~/snaptube
git add dist/index.html
git commit -m "Manual update from phone"
git push origin main
echo "ГОТОВО! Файл улетел на GitHub."
