import os
import logging
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from src.api.papers import router as papers_router

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(),
    ]
)

logger = logging.getLogger(__name__)

app = FastAPI(
    title="MICCAI 2025 Papers Visualization API",
    description="API for exploring MICCAI 2025 conference papers through interactive graph visualization",
    version="1.0.0"
)

# Configure CORS - allow all origins in development, specific origins in production
cors_origins = os.getenv("CORS_ORIGINS", "http://localhost:3000,http://localhost:3001,http://localhost:5173").split(",")
# Clean up any whitespace
cors_origins = [origin.strip() for origin in cors_origins if origin.strip()]

logger.info(f"CORS origins configured: {cors_origins}")

app.add_middleware(
    CORSMiddleware,
    allow_origins=cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include API routers
app.include_router(papers_router, prefix="/api")

@app.get("/")
async def root():
    logger.info("Root endpoint accessed")
    return {"message": "MICCAI 2025 Papers Visualization API"}

@app.get("/health")
async def health():
    logger.info("Health check endpoint accessed")
    return {"status": "healthy"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
