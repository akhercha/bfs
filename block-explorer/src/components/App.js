import React, { Component } from "react";
import "../styles/app.css";
import Block from "./Block";
import BlockDetails from "./BlockDetails";
import TXDetails from "./TXDetails";
import Account from "./Account";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import TX from "./TX";

const URL = "http://localhost:5000/";

class App extends Component {
  state = {
    blocks: [],
    txs_pool: [],
  };

  componentDidMount() {
    this.fetchBlocks();
    this.fetchTXsFromPool();
    this.binterval = setInterval(() => this.fetchBlocks(), 1000);
  }

  componentWillUnmount() {
    clearInterval(this.binterval);
    clearInterval(this.tinterval);
  }

  async fetchBlocks() {
    fetch(URL + "blocks")
      .then((response) => response.json())
      .then((block) => {
        let parsedBlocks = [];
        JSON.parse(block["blocks"]).forEach((block) => {
          parsedBlocks.push(JSON.parse(block));
        });
        this.setState({
          blocks: parsedBlocks,
        });
      });
  }

  async fetchTXsFromPool() {
    fetch(URL + "txs_pool")
      .then((response) => response.json())
      .then((tx) => {
        let parsedTXs = [];
        tx["txs_pool"].forEach((tx) => {
          parsedTXs.push(JSON.parse(tx));
        });
        this.setState({ txs_pool: parsedTXs });
      });
  }

  render() {
    const blockItems = [];
    this.state.blocks.forEach((block) => {
      blockItems.push(
        <Block
          number={block.number}
          hash={block.hash}
          nTxs={block.n_txs}
          volume={block.volume}
        />
      );
    });

    const txItems = [];
    this.state.txs_pool.forEach((tx) => {
      txItems.push(
        <TX
          number={tx.number}
          hash={tx.hash}
          fr={tx.fr}
          to={tx.to}
          value={tx.value}
        />
      );
    });

    return (
      <Router>
        <div class="home">
          <Switch>
            <Route path="/" exact>
              <div class="home-blocks">
                <div className="home-label">Blocks:</div>
                <div class="home-blocks-items">{blockItems}</div>
                <div className="home-label">TXs Pool:</div>
                <div className="home-txs-items">{txItems}</div>
              </div>
            </Route>
            <Route path="/block/:hash" component={BlockDetails} />
            <Route path="/acc/:hash" component={Account} />
            <Route path="/tx/:block_number/:hash" component={TXDetails} />
          </Switch>
        </div>
      </Router>
    );
  }
}

export default App;
