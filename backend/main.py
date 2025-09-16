from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from src.api.papers import router as papers_router

app = FastAPI(
    title="MICCAI 2025 Papers Visualization API",
    description="API for exploring MICCAI 2025 conference papers through interactive graph visualization",
    version="1.0.0"
)

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000", "http://localhost:3001", "http://localhost:5173"],  # React dev servers
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include API routers
app.include_router(papers_router, prefix="/api")

@app.get("/")
async def root():
    return {"message": "MICCAI 2025 Papers Visualization API"}

@app.get("/health")
async def health():
    return {"status": "healthy"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)