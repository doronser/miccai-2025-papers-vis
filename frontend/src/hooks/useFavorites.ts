import { useState, useEffect } from 'react';

const FAVORITES_STORAGE_KEY = 'miccai-papers-favorites';

export const useFavorites = () => {
  const [favorites, setFavorites] = useState<Set<string>>(new Set());

  // Load favorites from localStorage on mount
  useEffect(() => {
    const stored = localStorage.getItem(FAVORITES_STORAGE_KEY);
    if (stored) {
      try {
        const favoritesArray = JSON.parse(stored);
        setFavorites(new Set(favoritesArray));
      } catch (error) {
        console.error('Error loading favorites from localStorage:', error);
      }
    }
  }, []);

  // Save to localStorage whenever favorites change
  useEffect(() => {
    localStorage.setItem(FAVORITES_STORAGE_KEY, JSON.stringify(Array.from(favorites)));
  }, [favorites]);

  const toggleFavorite = (paperId: string) => {
    setFavorites(prev => {
      const newFavorites = new Set(prev);
      if (newFavorites.has(paperId)) {
        newFavorites.delete(paperId);
      } else {
        newFavorites.add(paperId);
      }
      return newFavorites;
    });
  };

  const addFavorite = (paperId: string) => {
    setFavorites(prev => new Set(prev.add(paperId)));
  };

  const removeFavorite = (paperId: string) => {
    setFavorites(prev => {
      const newFavorites = new Set(prev);
      newFavorites.delete(paperId);
      return newFavorites;
    });
  };

  const isFavorite = (paperId: string) => {
    return favorites.has(paperId);
  };

  const clearFavorites = () => {
    setFavorites(new Set());
  };

  return {
    favorites,
    toggleFavorite,
    addFavorite,
    removeFavorite,
    isFavorite,
    clearFavorites,
    favoritesCount: favorites.size
  };
};