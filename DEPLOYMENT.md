# Deployment Guide

This guide will help you deploy the MICCAI 2025 Papers Visualization webapp to a Hostinger VPS.

## Overview

The application consists of:
- **Frontend**: React app served by Nginx
- **Backend**: FastAPI app running with Gunicorn
- **Server**: Hostinger VPS with Ubuntu/Debian

## Prerequisites

- Hostinger VPS account
- Domain name (optional, can use VPS IP)
- SSH access to your VPS
- Basic knowledge of Linux commands

## Step 1: VPS Setup

### 1.1 Connect to Your VPS

```bash
ssh root@your-vps-ip
# or
ssh username@your-vps-ip
```

### 1.2 Update System

```bash
apt update && apt upgrade -y
```

### 1.3 Install Required Software

```bash
# Install Node.js 18
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
apt-get install -y nodejs

# Install Python 3.11
apt install -y python3.11 python3.11-venv python3.11-dev

# Install Nginx
apt install -y nginx

# Install Git
apt install -y git

# Install other dependencies
apt install -y build-essential curl
```

### 1.4 Configure Firewall

```bash
ufw allow 22    # SSH
ufw allow 80    # HTTP
ufw allow 443   # HTTPS
ufw enable
```

## Step 2: Deploy Backend

### 2.1 Clone Repository

```bash
cd /var/www
git clone https://github.com/your-username/miccai-2025-papers-vis.git
cd miccai-2025-papers-vis
```

### 2.2 Setup Backend

```bash
cd backend

# Create virtual environment
python3.11 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Install Gunicorn
pip install gunicorn
```

### 2.3 Create Gunicorn Service

Create `/etc/systemd/system/miccai-backend.service`:

```ini
[Unit]
Description=MICCAI Backend API
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/miccai-2025-papers-vis/backend
Environment="PATH=/var/www/miccai-2025-papers-vis/backend/.venv/bin"
ExecStart=/var/www/miccai-2025-papers-vis/backend/.venv/bin/gunicorn --workers 3 --bind 127.0.0.1:8000 main:app
Restart=always

[Install]
WantedBy=multi-user.target
```

### 2.4 Start Backend Service

```bash
systemctl daemon-reload
systemctl enable miccai-backend
systemctl start miccai-backend
systemctl status miccai-backend
```

## Step 3: Deploy Frontend

### 3.1 Build Frontend

```bash
cd /var/www/miccai-2025-papers-vis/frontend

# Install dependencies
npm install

# Build for production
npm run build
```

### 3.2 Configure Nginx

Create `/etc/nginx/sites-available/miccai-app`:

```nginx
server {
    listen 80;
    server_name your-domain.com your-vps-ip;

    # Frontend
    location / {
        root /var/www/miccai-2025-papers-vis/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Static files
    location /static {
        alias /var/www/miccai-2025-papers-vis/frontend/dist;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

### 3.3 Enable Site

```bash
ln -s /etc/nginx/sites-available/miccai-app /etc/nginx/sites-enabled/
rm /etc/nginx/sites-enabled/default
nginx -t
systemctl reload nginx
```

## Step 4: SSL Certificate (Optional but Recommended)

### 4.1 Install Certbot

```bash
apt install -y certbot python3-certbot-nginx
```

### 4.2 Get SSL Certificate

```bash
certbot --nginx -d your-domain.com
```

### 4.3 Auto-renewal

```bash
crontab -e
# Add this line:
0 12 * * * /usr/bin/certbot renew --quiet
```

## Step 5: Environment Configuration

### 5.1 Backend Environment

Create `/var/www/miccai-2025-papers-vis/backend/.env`:

```bash
CORS_ORIGINS=https://your-domain.com,http://your-vps-ip
```

### 5.2 Frontend Environment

Create `/var/www/miccai-2025-papers-vis/frontend/.env.production`:

```bash
VITE_API_BASE_URL=https://your-domain.com/api
```

## Step 6: Test Deployment

1. **Check Backend**: `curl http://your-vps-ip/api/health`
2. **Check Frontend**: Visit `http://your-vps-ip` in browser
3. **Check SSL**: Visit `https://your-domain.com` (if configured)

## Step 7: Monitoring and Maintenance

### 7.1 Log Monitoring

```bash
# Backend logs
journalctl -u miccai-backend -f

# Nginx logs
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log
```

### 7.2 Service Management

```bash
# Restart backend
systemctl restart miccai-backend

# Restart nginx
systemctl restart nginx

# Check status
systemctl status miccai-backend
systemctl status nginx
```

### 7.3 Updates

```bash
cd /var/www/miccai-2025-papers-vis
git pull origin main

# Rebuild frontend
cd frontend
npm run build

# Restart backend
systemctl restart miccai-backend
```

## Troubleshooting

### Common Issues

1. **502 Bad Gateway**:
   - Check if backend is running: `systemctl status miccai-backend`
   - Check backend logs: `journalctl -u miccai-backend`

2. **CORS Errors**:
   - Verify `CORS_ORIGINS` in backend environment
   - Check nginx proxy configuration

3. **Static Files Not Loading**:
   - Verify frontend build: `ls -la /var/www/miccai-2025-papers-vis/frontend/dist`
   - Check nginx static file configuration

4. **Permission Issues**:
   ```bash
   chown -R www-data:www-data /var/www/miccai-2025-papers-vis
   chmod -R 755 /var/www/miccai-2025-papers-vis
   ```

### Performance Optimization

1. **Enable Gzip Compression**:
   Add to nginx config:
   ```nginx
   gzip on;
   gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;
   ```

2. **Enable Caching**:
   ```nginx
   location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg)$ {
       expires 1y;
       add_header Cache-Control "public, immutable";
   }
   ```

3. **Increase Gunicorn Workers**:
   Edit `/etc/systemd/system/miccai-backend.service`:
   ```ini
   ExecStart=/var/www/miccai-2025-papers-vis/backend/.venv/bin/gunicorn --workers 4 --bind 127.0.0.1:8000 main:app
   ```

## Security Considerations

1. **Firewall**: Only open necessary ports
2. **SSH**: Use key-based authentication
3. **Updates**: Keep system and dependencies updated
4. **SSL**: Always use HTTPS in production
5. **Environment Variables**: Never commit sensitive data

## Backup Strategy

1. **Code Backup**: Use Git repository
2. **Data Backup**: Regular backups of `/var/www/miccai-2025-papers-vis`
3. **Database Backup**: If using database, regular dumps
4. **SSL Certificates**: Backup certificate files

This setup provides a robust, scalable deployment on Hostinger VPS! 🚀
