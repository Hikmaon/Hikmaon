import React from "react";
import { useWallet } from "../hooks/useWallet";

const WalletAuth = () => {
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
      <div className="relative overflow-hidden rounded-2xl backdrop-blur-xl bg-white/10 border border-white/20 p-6 max-w-md">
        <div className="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-red-500/10"></div>
        <div className="relative z-10 text-center">
          <div className="p-3 rounded-xl bg-gradient-to-r from-orange-500/20 to-red-500/20 inline-block mb-4">
            <span className="text-3xl">⚠️</span>
          </div>
          <h3 className="text-xl font-bold text-white mb-2">
            MetaMask Required
          </h3>
          <p className="text-gray-300 mb-4">
            Please install MetaMask to interact with HIKMAON
          </p>
          <a
            href="https://metamask.io/"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 bg-gradient-to-r from-orange-500 to-red-600 text-white px-6 py-3 rounded-xl font-medium transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-orange-500/25"
          >
            <span>🦊</span>
            Install MetaMask
          </a>
        </div>
      </div>
    );
  }

  return (
    <div className="relative overflow-hidden rounded-2xl backdrop-blur-xl bg-white/10 border border-white/20 p-6 max-w-md">
      <div
        className={`absolute inset-0 ${
          isConnected
            ? "bg-gradient-to-br from-green-500/10 to-emerald-500/10"
            : "bg-gradient-to-br from-blue-500/10 to-purple-500/10"
        }`}
      ></div>

      <div className="relative z-10">
        <div className="flex items-center gap-4 mb-4">
          <div
            className={`p-3 rounded-xl ${
              isConnected
                ? "bg-gradient-to-r from-green-500/20 to-emerald-500/20"
                : "bg-gradient-to-r from-blue-500/20 to-purple-500/20"
            }`}
          >
            <span className="text-2xl">{isConnected ? "🟢" : "🔴"}</span>
          </div>
          <div className="flex-1">
            <h3 className="text-lg font-bold text-white">
              {isConnected ? "Wallet Connected" : "Connect Your Wallet"}
            </h3>
            {!isConnected && (
              <p className="text-gray-300 text-sm">
                Connect MetaMask to interact with the blockchain
              </p>
            )}
          </div>
        </div>

        {isConnected ? (
          <div className="space-y-4">
            <div className="flex items-center gap-2 p-3 bg-white/10 rounded-xl backdrop-blur-sm border border-white/20">
              <span className="text-sm font-medium text-gray-400">
                Account:
              </span>
              <code className="text-white font-mono text-sm bg-white/10 px-2 py-1 rounded border border-white/20">
                {account?.slice(0, 6)}...{account?.slice(-4)}
              </code>
              <button
                onClick={() => navigator.clipboard.writeText(account)}
                className="text-gray-400 hover:text-white text-sm transition-colors duration-300 p-1 rounded hover:bg-white/10"
                title="Copy full address"
              >
                📋
              </button>
            </div>

            <div className="p-3 bg-white/10 rounded-xl backdrop-blur-sm border border-white/20">
              <h4 className="text-sm font-medium text-green-300 mb-2">
                🔓 Authenticated Features Available:
              </h4>
              <ul className="space-y-1 text-xs text-gray-300">
                <li>✅ Issue & Verify Certificates</li>
                <li>✅ Transfer Tokens Securely</li>
                <li>✅ Mine Blocks</li>
                <li>✅ Access Full Blockchain Features</li>
              </ul>
            </div>

            <button
              onClick={disconnectWallet}
              className="w-full bg-gradient-to-r from-red-500 to-red-600 text-white px-4 py-3 rounded-xl font-medium transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-red-500/25"
            >
              Disconnect Wallet
            </button>
          </div>
        ) : (
          <div className="space-y-4">
            {error && (
              <div className="p-3 bg-red-500/20 border border-red-500/30 text-red-300 rounded-xl backdrop-blur-sm">
                {error}
              </div>
            )}
            <button
              onClick={connectWallet}
              disabled={isLoading}
              className="w-full relative overflow-hidden bg-gradient-to-r from-blue-500 to-purple-600 text-white px-4 py-3 rounded-xl font-medium transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-blue-500/25 disabled:opacity-50 disabled:cursor-not-allowed group"
            >
              <div className="absolute inset-0 bg-gradient-to-r from-blue-600 to-purple-700 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
              <span className="relative z-10 flex items-center justify-center gap-2">
                {isLoading ? (
                  <>
                    <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                    Connecting...
                  </>
                ) : (
                  <>
                    <span>🦊</span>
                    Connect MetaMask
                  </>
                )}
              </span>
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default WalletAuth;
