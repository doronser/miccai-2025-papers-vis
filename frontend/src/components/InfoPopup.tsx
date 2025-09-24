import React from 'react';

interface InfoPopupProps {
    isOpen: boolean;
    onClose: () => void;
}

export const InfoPopup: React.FC<InfoPopupProps> = ({ isOpen, onClose }) => {
    if (!isOpen) return null;

    return (
        <>
            {/* Backdrop */}
            <div
                className="info-popup-backdrop"
                onClick={onClose}
                aria-hidden="true"
            />

            {/* Popup */}
            <div className="info-popup">
                <div className="info-popup-header">
                    <h3>About This Project</h3>
                    <button
                        className="info-popup-close"
                        onClick={onClose}
                        aria-label="Close info popup"
                    >
                        ×
                    </button>
                </div>
                <div className="info-popup-content">
                    <p>
                        The goal of this project is to make browsing MICCAI 2025 papers easier.
                    </p>

                    <p>
                        <strong>How it works:</strong>
                    </p>
                    <ul>
                        <li>MICCAI abstracts and info were scraped from the <a href="https://papers.miccai.org/miccai-2025/" target="_blank" rel="noopener noreferrer">MICCAI website</a>.</li>
                        <li>Embeddings were generated using SciBeRT.</li>
                        <li>t-SNE was used for visualization.</li>
                        <li>Cosine similarity was used to find similar papers.</li>
                    </ul>

                    <p>
                        <strong>How it was built:</strong>
                    </p>
                    <ul>
                        <li>This project was built using Github's <a href="https://github.com/github/spec-kit" target="_blank" rel="noopener noreferrer">Spec-Kit</a></li>
                        <li>Backend (scraping, embedding, similarity) was built using Python and FastAPI.</li>
                        <li>Frontend was built using React.</li>
                    </ul>

                    <p>
                        Built by Doron Serebro.<br />
                        Contact: doronser@gmail.com
                    </p>
                </div>
            </div>
        </>
    );
};

export default InfoPopup;
