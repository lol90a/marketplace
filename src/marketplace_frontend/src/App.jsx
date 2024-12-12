import React, { useState } from "react";
import { marketplace_backend } from "declarations/marketplace_backend"; // Auto-generated after deploying backend

function App() {
  // State variables
  const [assetId, setAssetId] = useState("");
  const [price, setPrice] = useState("");
  const [message, setMessage] = useState("");
  const [buyAssetId, setBuyAssetId] = useState("");

  // Function to list an asset
  const listAsset = async () => {
    try {
      const result = await marketplace_backend.list_asset(assetId, price);
      setMessage(result);
    } catch (error) {
      console.error("Error listing asset:", error);
      setMessage("Failed to list asset. See console for details.");
    }
  };

  // Function to buy an asset
  const buyAsset = async (assetId) => {
    try {
      const now = new Date();
      const expiry = new Date(now.getTime() + 5 * 60 * 1000); // Valid for 5 minutes
      const result = await marketplace_backend.buy_asset(assetId, expiry.toISOString());
      console.log(result);
    } catch (error) {
      console.error("Error buying asset:", error);
    }
  };
  

  return (
    <div style={styles.container}>
      <h1 style={styles.title}>Crypto Exchange Marketplace</h1>

      {/* List an Asset */}
      <div style={styles.section}>
        <h2>List an Asset</h2>
        <input
          type="text"
          placeholder="Asset ID"
          value={assetId}
          onChange={(e) => setAssetId(e.target.value)}
          style={styles.input}
        />
        <input
          type="number"
          placeholder="Price"
          value={price}
          onChange={(e) => setPrice(e.target.value)}
          style={styles.input}
        />
        <button onClick={listAsset} style={styles.button}>
          List Asset
        </button>
      </div>

      {/* Buy an Asset */}
      <div style={styles.section}>
        <h2>Buy an Asset</h2>
        <input
          type="text"
          placeholder="Asset ID to Buy"
          value={buyAssetId}
          onChange={(e) => setBuyAssetId(e.target.value)}
          style={styles.input}
        />
        <button onClick={buyAsset} style={styles.button}>
          Buy Asset
        </button>
      </div>

      {/* Display Messages */}
      <div style={styles.output}>
        <h3>Response:</h3>
        <p>{message}</p>
      </div>
    </div>
  );
}

// Basic inline CSS for styling
const styles = {
  container: {
    padding: "20px",
    fontFamily: "Arial, sans-serif",
    maxWidth: "600px",
    margin: "auto",
  },
  title: {
    textAlign: "center",
    color: "#333",
  },
  section: {
    marginBottom: "20px",
  },
  input: {
    display: "block",
    width: "100%",
    marginBottom: "10px",
    padding: "8px",
    fontSize: "16px",
  },
  button: {
    backgroundColor: "#4CAF50",
    color: "white",
    padding: "10px 20px",
    fontSize: "16px",
    cursor: "pointer",
    border: "none",
    borderRadius: "5px",
  },
  output: {
    marginTop: "20px",
    backgroundColor: "#f9f9f9",
    padding: "10px",
    borderRadius: "5px",
    border: "1px solid #ddd",
  },
};

export default App;
