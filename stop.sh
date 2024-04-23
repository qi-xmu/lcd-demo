ssh pi "ps aux | grep lcd-demo | head -1 | awk '{print \$2}' | xargs kill -9"
ssh pi "ps aux | grep lcd-demo | head -1"