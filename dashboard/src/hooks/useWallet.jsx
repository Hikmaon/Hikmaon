/* eslint-disable react-refresh/only-export-components */

// src/hooks/useWallet.jsx
import { useState, useEffect, createContext, useContext } from "react";

const API_BASE = "http://127.0.0.1:3000";

// Wallet Context
const WalletContext = createContext();

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error("useWallet must be used within WalletProvider");
  }
  return context;
};

export const WalletProvider = ({ children }) => {
  const [account, setAccount] = useState(null);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  // Check if MetaMask is installed
  const isMetaMaskInstalled = () => {
    return typeof window.ethereum !== "undefined" && window.ethereum.isMetaMask;
  };

  // Connect wallet
  const connectWallet = async () => {
    if (!isMetaMaskInstalled()) {
      setError("MetaMask is not installed. Please install it to continue.");
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Request account access
      const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
      });

      if (accounts.length > 0) {
        const userAccount = accounts[0];
        setAccount(userAccount);

        // Authenticate with backend
        await authenticateWithBackend(userAccount);

        setIsConnected(true);

        // Listen for account changes
        window.ethereum.on("accountsChanged", handleAccountsChanged);

        // Store in localStorage for persistence (wallet only, not sensitive data)
        localStorage.setItem("walletAccount", userAccount);
      }
    } catch (err) {
      console.error("Failed to connect wallet:", err);
      setError("Failed to connect wallet. Please try again.");
    } finally {
      setIsLoading(false);
    }
  };

  // Authenticate with backend using message signing
  const authenticateWithBackend = async (userAccount) => {
    try {
      console.log("Starting authentication for address:", userAccount);

      // Get nonce from backend
      const nonceResponse = await fetch(`${API_BASE}/auth/nonce`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ address: userAccount }),
      });

      if (!nonceResponse.ok) {
        throw new Error(`Failed to get nonce: ${nonceResponse.status}`);
      }

      const { nonce } = await nonceResponse.json();
      console.log("Received nonce:", nonce);

      // Create message to sign
      const message = `Please sign this message to authenticate with Hikmalayer.\n\nNonce: ${nonce}\nAddress: ${userAccount}`;

      // Request signature from MetaMask
      const signature = await window.ethereum.request({
        method: "personal_sign",
        params: [message, userAccount],
      });

      console.log("Got signature, verifying with backend...");

      // Verify signature with backend
      const authResponse = await fetch(`${API_BASE}/auth/verify`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          address: userAccount,
          message,
          signature,
          nonce,
        }),
      });

      if (!authResponse.ok) {
        const errorText = await authResponse.text();
        throw new Error(
          `Authentication failed: ${authResponse.status} - ${errorText}`
        );
      }

      const { token } = await authResponse.json();

      // Store auth token in memory for security (don't use localStorage for sensitive data)
      window.authToken = token;

      console.log("Successfully authenticated with backend");
    } catch (err) {
      console.error("Backend authentication failed:", err);
      throw err;
    }
  };

  // Disconnect wallet
  const disconnectWallet = () => {
    setAccount(null);
    setIsConnected(false);

    // Clear stored data
    localStorage.removeItem("walletAccount");
    delete window.authToken;

    // Remove MetaMask event listeners
    if (window.ethereum) {
      window.ethereum.removeListener("accountsChanged", handleAccountsChanged);
    }
  };

  // Handle account changes
  const handleAccountsChanged = (accounts) => {
    if (accounts.length === 0) {
      // User disconnected wallet
      disconnectWallet();
    } else {
      // User switched accounts
      const newAccount = accounts[0];
      setAccount(newAccount);
      localStorage.setItem("walletAccount", newAccount);

      // Re-authenticate with new account
      authenticateWithBackend(newAccount).catch((err) => {
        console.error("Failed to re-authenticate with new account:", err);
        setError("Failed to authenticate with new account");
      });
    }
  };

  // Check for existing connection on load
  useEffect(() => {
    const checkExistingConnection = async () => {
      const savedAccount = localStorage.getItem("walletAccount");

      if (savedAccount && isMetaMaskInstalled()) {
        try {
          // Verify the account is still connected in MetaMask
          const accounts = await window.ethereum.request({
            method: "eth_accounts",
          });

          if (accounts.includes(savedAccount)) {
            setAccount(savedAccount);

            // Try to re-authenticate
            await authenticateWithBackend(savedAccount);
            setIsConnected(true);

            // Set up event listeners
            window.ethereum.on("accountsChanged", handleAccountsChanged);
          } else {
            // Account no longer available, clear storage
            disconnectWallet();
          }
        } catch (err) {
          console.error("Failed to check existing connection:", err);
          disconnectWallet();
        }
      }
    };

    checkExistingConnection();

    // Cleanup event listeners on unmount
    return () => {
      if (window.ethereum) {
        window.ethereum.removeListener(
          "accountsChanged",
          handleAccountsChanged
        );
      }
    };
  }, []);

  // Sign message utility
  const signMessage = async (message) => {
    if (!account) {
      throw new Error("No wallet connected");
    }

    try {
      const signature = await window.ethereum.request({
        method: "personal_sign",
        params: [message, account],
      });
      return signature;
    } catch (err) {
      console.error("Failed to sign message:", err);
      throw err;
    }
  };

  // Get authenticated API headers
  const getAuthHeaders = () => {
    const token = window.authToken;
    return token ? { Authorization: `Bearer ${token}` } : {};
  };

  const value = {
    account,
    isConnected,
    isLoading,
    error,
    isMetaMaskInstalled,
    connectWallet,
    disconnectWallet,
    signMessage,
    getAuthHeaders,
  };

  return (
    <WalletContext.Provider value={value}>{children}</WalletContext.Provider>
  );
};

// Wallet connection component
export const WalletConnector = () => {
  const {
    account,
    isConnected,
    isLoading,
    error,
    isMetaMaskInstalled,
    connectWallet,
    disconnectWallet,
  } = useWallet();

  if (!isMetaMaskInstalled()) {
    return (
      <div className="wallet-connector error">
        <p>MetaMask not detected</p>
        <a
          href="https://metamask.io/"
          target="_blank"
          rel="noopener noreferrer"
          className="install-button"
        >
          Install MetaMask
        </a>
      </div>
    );
  }

  if (isConnected) {
    return (
      <div className="wallet-connector connected">
        <div className="account-info">
          <span className="account-label">Connected:</span>
          <span className="account-address">
            {account?.slice(0, 6)}...{account?.slice(-4)}
          </span>
        </div>
        <button onClick={disconnectWallet} className="disconnect-button">
          Disconnect
        </button>
      </div>
    );
  }

  return (
    <div className="wallet-connector">
      {error && <div className="error-message">{error}</div>}
      <button
        onClick={connectWallet}
        disabled={isLoading}
        className="connect-button"
      >
        {isLoading ? "Connecting..." : "Connect MetaMask"}
      </button>
    </div>
  );
};
