// src/hooks/useAuthenticatedApi.js
import { useWallet } from "./useWallet";

const API_BASE = "http://127.0.0.1:3000";

export const useAuthenticatedApi = () => {
  const { getAuthHeaders, isConnected, account } = useWallet();

  const authenticatedFetch = async (endpoint, options = {}) => {
    if (!isConnected) {
      throw new Error("Wallet not connected");
    }

    const authHeaders = getAuthHeaders();

    try {
      const response = await fetch(`${API_BASE}${endpoint}`, {
        ...options,
        headers: {
          "Content-Type": "application/json",
          ...authHeaders,
          ...options.headers,
        },
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP ${response.status}: ${errorText}`);
      }

      return response;
    } catch (error) {
      console.error(`Authenticated API Error for ${endpoint}:`, error);
      throw error;
    }
  };

  return {
    authenticatedFetch,
    isConnected,
    account,
  };
};
