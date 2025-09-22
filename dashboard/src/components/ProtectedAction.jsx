import React from "react";
import { useWallet } from "../hooks/useWallet";

const ProtectedAction = ({ children, fallback, requireAuth = true }) => {
  const { isConnected } = useWallet();

  if (requireAuth && !isConnected) {
    return (
      fallback || (
        <div className="group relative overflow-hidden rounded-2xl backdrop-blur-xl bg-white/10 border border-white/20 p-6 hover:bg-white/15 transition-all duration-500">
          <div className="absolute inset-0 bg-gradient-to-br from-yellow-500/10 to-orange-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
          <div className="relative z-10 text-center">
            <div className="p-3 rounded-xl bg-gradient-to-r from-yellow-500/20 to-orange-500/20 inline-block mb-4">
              <span className="text-3xl">🔒</span>
            </div>
            <h3 className="text-xl font-bold text-white mb-2">
              Authentication Required
            </h3>
            <p className="text-gray-300">
              Please connect your wallet to use this feature
            </p>
          </div>
        </div>
      )
    );
  }

  return children;
};

export default ProtectedAction;
