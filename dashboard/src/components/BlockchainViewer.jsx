import React, { useState, useEffect } from "react";
import {
  getBlocks,
  getPendingTransactions,
  getBlockByIndex,
  validateBlock,
} from "../api";

const BlockchainViewer = ({ refreshTrigger }) => {
  const [blocks, setBlocks] = useState([]);
  const [pendingTxs, setPendingTxs] = useState([]);
  const [loading, setLoading] = useState(false);
  const [selectedBlock, setSelectedBlock] = useState(null);
  const [blockValidation, setBlockValidation] = useState(null);
  const [validatingBlock, setValidatingBlock] = useState(null);

  const fetchData = async () => {
    setLoading(true);
    try {
      const [blocksResponse, pendingResponse] = await Promise.all([
        getBlocks(),
        getPendingTransactions(),
      ]);

      setBlocks(blocksResponse.data);
      setPendingTxs(pendingResponse.data);
    } catch (error) {
      console.error("Error fetching blockchain data:", error);
    }
    setLoading(false);
  };

  const handleBlockClick = async (blockIndex) => {
    try {
      const response = await getBlockByIndex(blockIndex);
      setSelectedBlock({
        index: blockIndex,
        data: response.data,
      });
    } catch (error) {
      console.error("Error fetching block details:", error);
    }
  };

  const handleValidateBlock = async (blockIndex) => {
    setValidatingBlock(blockIndex);
    setBlockValidation(null);

    try {
      const response = await validateBlock(blockIndex);
      setBlockValidation(response.data);
    } catch (error) {
      console.error("Error validating block:", error);
      setBlockValidation({
        is_valid: false,
        message: "Validation failed",
        details: error.message,
      });
    }

    setValidatingBlock(null);
  };

  useEffect(() => {
    fetchData();
  }, [refreshTrigger]);

  const formatTimestamp = (timestamp) => {
    return new Date(timestamp).toLocaleString();
  };

  const truncateHash = (hash, length = 16) => {
    if (!hash) return "N/A";
    return hash.length > length ? `${hash.slice(0, length)}...` : hash;
  };

  return (
    <div className="relative overflow-hidden rounded-2xl backdrop-blur-xl bg-white/10 border border-white/20 p-6">
      <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/10 to-blue-500/10"></div>

      <div className="relative z-10">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center">
            <div className="p-2 rounded-xl bg-gradient-to-r from-indigo-500/20 to-blue-500/20 mr-3">
              <span className="text-2xl">🔍</span>
            </div>
            <h2 className="text-xl font-bold text-white">
              Blockchain Explorer
            </h2>
          </div>
          <button
            className="relative overflow-hidden bg-gradient-to-r from-indigo-500 to-blue-600 text-white px-4 py-2 rounded-xl font-medium transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-indigo-500/25 disabled:opacity-50 flex items-center gap-2 group"
            onClick={fetchData}
            disabled={loading}
          >
            <div className="absolute inset-0 bg-gradient-to-r from-indigo-600 to-blue-700 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            <span className={`relative z-10 ${loading ? "animate-spin" : ""}`}>
              🔄
            </span>
            <span className="relative z-10">
              {loading ? "Loading..." : "Refresh"}
            </span>
          </button>
        </div>

        {/* Pending Transactions Section */}
        {pendingTxs.length > 0 && (
          <div className="mb-6">
            <h3 className="text-lg font-semibold text-gray-200 mb-3 flex items-center gap-2">
              <span className="animate-pulse">⏳</span>
              <span>Pending Transactions ({pendingTxs.length})</span>
            </h3>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {pendingTxs.map((tx, idx) => (
                <div
                  key={idx}
                  className="bg-gradient-to-r from-orange-500/20 to-yellow-500/20 border border-orange-500/30 rounded-xl p-3 backdrop-blur-sm hover:scale-[1.02] transition-all duration-300"
                >
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium text-orange-300">
                      Transaction #{idx + 1}
                    </span>
                    <span className="text-xs text-orange-400 bg-orange-500/20 px-2 py-1 rounded-lg backdrop-blur-sm animate-pulse">
                      PENDING
                    </span>
                  </div>
                  <code className="text-xs text-orange-200 break-all block bg-white/10 p-3 rounded-lg border border-white/20 backdrop-blur-sm">
                    {tx}
                  </code>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Blocks Section */}
        <div>
          <h3 className="text-lg font-semibold text-gray-200 mb-3 flex items-center gap-2">
            <span>📦</span>
            <span>Blockchain Blocks ({blocks.length})</span>
          </h3>

          {blocks.length === 0 ? (
            <div className="text-center py-8 text-gray-400">
              <span className="text-4xl mb-2 block animate-bounce">📭</span>
              <p>No blocks found. Start by mining your first block!</p>
            </div>
          ) : (
            <div className="space-y-3 max-h-96 overflow-y-auto">
              {blocks.map((block, idx) => {
                const isGenesis = idx === 0;
                const blockData =
                  typeof block === "string" ? block : JSON.stringify(block);

                const indexMatch = blockData.match(/index: (\d+)/);
                const timestampMatch = blockData.match(/timestamp: ([^,}]+)/);
                const hashMatch = blockData.match(/hash: "([^"]+)"/);
                const nonceMatch = blockData.match(/nonce: (\d+)/);

                return (
                  <div
                    key={idx}
                    className={`group relative overflow-hidden border rounded-xl p-4 transition-all duration-300 hover:shadow-lg cursor-pointer backdrop-blur-sm ${
                      isGenesis
                        ? "bg-gradient-to-r from-green-500/20 to-emerald-500/20 border-green-500/30 hover:scale-[1.02]"
                        : "bg-gradient-to-r from-gray-500/20 to-slate-500/20 border-gray-500/30 hover:scale-[1.02]"
                    }`}
                    style={{ animationDelay: `${idx * 100}ms` }}
                  >
                    <div className="absolute inset-0 bg-gradient-to-r from-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

                    <div className="relative z-10">
                      <div className="flex items-center justify-between mb-3">
                        <div className="flex items-center gap-2">
                          <span
                            className={`text-lg transition-transform duration-300 group-hover:scale-125 ${
                              isGenesis ? "text-green-400" : "text-blue-400"
                            }`}
                          >
                            {isGenesis ? "🌟" : "📦"}
                          </span>
                          <span
                            className={`font-medium ${
                              isGenesis ? "text-green-300" : "text-gray-200"
                            }`}
                          >
                            {isGenesis ? "Genesis Block" : `Block #${idx}`}
                          </span>
                        </div>

                        <div className="flex gap-2">
                          <button
                            className="bg-gradient-to-r from-blue-500/80 to-cyan-500/80 text-white px-3 py-1 rounded-lg text-xs transition-all duration-300 hover:scale-105 backdrop-blur-sm"
                            onClick={() => handleBlockClick(idx)}
                          >
                            📋 Details
                          </button>
                          <button
                            className="bg-gradient-to-r from-teal-500/80 to-green-500/80 text-white px-3 py-1 rounded-lg text-xs transition-all duration-300 hover:scale-105 disabled:opacity-50 backdrop-blur-sm"
                            onClick={() => handleValidateBlock(idx)}
                            disabled={validatingBlock === idx}
                          >
                            {validatingBlock === idx ? "⏳" : "✅"} Validate
                          </button>
                        </div>
                      </div>

                      <div className="grid grid-cols-2 gap-4 text-xs text-gray-300 mb-2">
                        <div>
                          <span className="font-medium text-gray-400">
                            Index:
                          </span>{" "}
                          <span className="text-white">
                            {indexMatch ? indexMatch[1] : "N/A"}
                          </span>
                        </div>
                        <div>
                          <span className="font-medium text-gray-400">
                            Nonce:
                          </span>{" "}
                          <span className="text-white">
                            {nonceMatch ? nonceMatch[1] : "N/A"}
                          </span>
                        </div>
                        <div>
                          <span className="font-medium text-gray-400">
                            Timestamp:
                          </span>{" "}
                          <span className="text-white">
                            {timestampMatch
                              ? formatTimestamp(
                                  timestampMatch[1].replace(/['"]/g, "")
                                )
                              : "N/A"}
                          </span>
                        </div>
                        <div>
                          <span className="font-medium text-gray-400">
                            Hash:
                          </span>{" "}
                          <span className="text-white font-mono">
                            {hashMatch ? truncateHash(hashMatch[1]) : "N/A"}
                          </span>
                        </div>
                      </div>

                      <details className="mt-2 group">
                        <summary className="cursor-pointer text-xs text-gray-400 hover:text-gray-200 transition-colors duration-300">
                          🔍 View Raw Block Data
                        </summary>
                        <code className="text-xs text-gray-300 break-all block bg-white/10 p-3 rounded-lg border border-white/20 mt-2 max-h-32 overflow-y-auto backdrop-blur-sm">
                          {blockData}
                        </code>
                      </details>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Block Details Modal */}
        {selectedBlock && (
          <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4">
            <div className="relative overflow-hidden bg-white/10 backdrop-blur-xl border border-white/20 rounded-2xl max-w-2xl w-full max-h-96 overflow-y-auto">
              <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/10 to-purple-500/10"></div>

              <div className="relative z-10 p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-bold text-white">
                    Block #{selectedBlock.index} Details
                  </h3>
                  <button
                    className="text-gray-400 hover:text-white text-2xl transition-colors duration-300 hover:rotate-90 transform"
                    onClick={() => setSelectedBlock(null)}
                  >
                    ×
                  </button>
                </div>
                <div className="bg-white/10 p-4 rounded-xl backdrop-blur-sm border border-white/20">
                  <code className="text-sm text-gray-200 whitespace-pre-wrap break-all">
                    {selectedBlock.data || "No data available"}
                  </code>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Block Validation Results */}
        {blockValidation && (
          <div
            className={`mt-4 p-4 rounded-xl backdrop-blur-sm border transition-all duration-500 ${
              blockValidation.is_valid
                ? "bg-green-500/20 text-green-300 border-green-500/30"
                : "bg-red-500/20 text-red-300 border-red-500/30"
            } animate-pulse`}
          >
            <div className="flex items-center gap-2 mb-2">
              <span className="text-lg">
                {blockValidation.is_valid ? "✅" : "❌"}
              </span>
              <span className="font-medium">{blockValidation.message}</span>
            </div>
            {blockValidation.details && (
              <p className="text-sm">{blockValidation.details}</p>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default BlockchainViewer;
