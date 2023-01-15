#!/bin/bash
read -p 'Enter message: ' message

git pull origin main
git add .
git commit -m "$message"
git push origin main
