# Deployment Guide

This guide will help you deploy the MICCAI 2025 Papers Visualization webapp to production.

## Overview

The application consists of:
- **Frontend**: React app deployed to Vercel
- **Backend**: FastAPI app deployed to Railway/Render/Heroku

## Prerequisites

- GitHub account
- Vercel account (free tier available)
- Railway/Render/Heroku account (free tier available)
- Domain name (optional)

## Step 1: Deploy Backend

### Option A: Railway (Recommended)

1. **Connect to Railway**:
   ```bash
   npm install -g @railway/cli
   railway login
   ```

2. **Deploy**:
   ```bash
   railway init
   railway up
   ```

3. **Set Environment Variables**:
   - Go to Railway dashboard
   - Add `CORS_ORIGINS` = `https://your-frontend.vercel.app`

### Option B: Render

1. **Connect Repository**:
   - Go to [Render Dashboard](https://dashboard.render.com)
   - Click "New" → "Web Service"
   - Connect your GitHub repository

2. **Configure**:
   - Build Command: `pip install -r requirements.txt`
   - Start Command: `python main.py`
   - Environment: `Python 3`

3. **Set Environment Variables**:
   - `CORS_ORIGINS` = `https://your-frontend.vercel.app`

### Option C: Heroku

1. **Install Heroku CLI**:
   ```bash
   # Install Heroku CLI
   heroku login
   ```

2. **Deploy**:
   ```bash
   heroku create your-app-name
   git push heroku main
   ```

3. **Set Environment Variables**:
   ```bash
   heroku config:set CORS_ORIGINS=https://your-frontend.vercel.app
   ```

## Step 2: Deploy Frontend to Vercel

1. **Connect to Vercel**:
   - Go to [Vercel Dashboard](https://vercel.com/dashboard)
   - Click "New Project"
   - Import your GitHub repository

2. **Configure Build Settings**:
   - Framework Preset: `Vite`
   - Root Directory: `frontend`
   - Build Command: `npm run build`
   - Output Directory: `dist`

3. **Set Environment Variables**:
   - `VITE_API_BASE_URL` = `https://your-backend.railway.app/api`

4. **Deploy**:
   - Click "Deploy"
   - Vercel will automatically build and deploy

## Step 3: Update CORS Settings

After both deployments are complete:

1. **Get Frontend URL**: Copy your Vercel deployment URL
2. **Update Backend CORS**:
   - Go to your backend deployment dashboard
   - Update `CORS_ORIGINS` environment variable
   - Add your Vercel URL: `https://your-app.vercel.app`

## Step 4: Test Deployment

1. **Visit Frontend**: Go to your Vercel URL
2. **Check API**: Verify the graph loads correctly
3. **Test Features**: Try searching, clicking papers, etc.

## Environment Variables Reference

### Frontend (Vercel)
```bash
VITE_API_BASE_URL=https://your-backend.railway.app/api
```

### Backend (Railway/Render/Heroku)
```bash
CORS_ORIGINS=https://your-frontend.vercel.app
```

## Troubleshooting

### Common Issues

1. **CORS Errors**:
   - Ensure `CORS_ORIGINS` includes your Vercel URL
   - Check that URLs don't have trailing slashes

2. **API Not Loading**:
   - Verify `VITE_API_BASE_URL` is correct
   - Check backend is running and accessible

3. **Build Failures**:
   - Check build logs in deployment dashboard
   - Ensure all dependencies are in `package.json`

### Performance Optimization

1. **Enable Caching**:
   - Vercel automatically caches static assets
   - Backend can implement Redis caching

2. **CDN**:
   - Vercel provides global CDN automatically
   - Consider CloudFlare for additional optimization

## Custom Domain (Optional)

1. **Add Domain to Vercel**:
   - Go to Project Settings → Domains
   - Add your custom domain
   - Follow DNS configuration instructions

2. **Update CORS**:
   - Add custom domain to `CORS_ORIGINS`
   - Update `VITE_API_BASE_URL` if needed

## Monitoring

1. **Vercel Analytics**:
   - Enable in Project Settings
   - Monitor performance and usage

2. **Backend Monitoring**:
   - Use platform-specific monitoring
   - Set up alerts for downtime

## Security Considerations

1. **Environment Variables**:
   - Never commit `.env` files
   - Use platform environment variable settings

2. **CORS Configuration**:
   - Only allow necessary origins
   - Avoid wildcard origins in production

3. **Rate Limiting**:
   - Consider implementing rate limiting
   - Monitor API usage patterns
