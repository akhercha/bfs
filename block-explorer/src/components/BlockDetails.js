//import React from "react";
import React, { Component } from "react";
import question from "../svgs/question.svg";
import "../styles/blockdetails.css";
import { Link } from "react-router-dom";
import TX from "./TX";

class BlockDetails extends Component {
  constructor(props) {
    super(props);
    this.state = {
      block: [],
      txs: [],
    };
  }

  componentDidMount() {
    fetch("http://localhost:5000/block/" + this.props.match.params.hash)
      .then((response) => response.json())
      .then((block) => {
        this.setState({ block: block, txs: block.txs });
      });
  }
  render() {
    var isMined = this.state.block.mined;
    var d = new Date(this.state.block.time * 1000);

    return (
      <div class="bd">
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Hash:</div>
          </div>
          <div class="bd-row-content" id="bd-row-content-hash">
            {this.state.block.hash}
          </div>
        </div>
        <div class="bd-row-divider"></div>
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Block Height:</div>
          </div>
          <div class="bd-row-content" id="bd-row-content-height">
            {this.state.block.number}
          </div>
        </div>
        <div class="bd-row-divider"></div>

        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Tx Root:</div>
          </div>
          <div class="bd-row-content" id="bd-row-content-hash">
            {this.state.block.root}
          </div>
        </div>
        <div class="bd-row-divider"></div>
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Timestamp:</div>
          </div>
          <div class="bd-row-content">{d.toString()} </div>
        </div>
        <div class="bd-row-divider"></div>
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Number:</div>
          </div>
          <div class="bd-row-content">
            {this.state.block.n_txs} transactions
          </div>
        </div>
        <div class="bd-row-divider"></div>
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Is Mined:</div>
          </div>
          <div class="bd-row-content" id="bd-row-content-hash">
            {isMined ? (
              <div class="bd-isMined-true">true</div>
            ) : (
              <div class="bd-isMined-false">false</div>
            )}
          </div>
        </div>

        {isMined && (
          <div>
            <div class="bd-row-divider"></div>
            <div class="bd-row">
              <div class="bd-row-title">
                <img class="bd-row-title-question" src={question} />
                <div class="bd-row-title-label">Block Reward:</div>
              </div>
              <div class="bd-row-content">{this.state.block.reward} Ether</div>
            </div>
            <div class="bd-row-divider"></div>
            <div class="bd-row">
              <div class="bd-row-title">
                <img class="bd-row-title-question" src={question} />
                <div class="bd-row-title-label">Difficulty:</div>
              </div>
              <div class="bd-row-content" id="bd-row-content-hash">
                {this.state.block.diff}
              </div>
            </div>
          </div>
        )}
        <div class="bd-row-divider"></div>
        <div class="bd-row">
          <div class="bd-row-title">
            <img class="bd-row-title-question" src={question} />
            <div class="bd-row-title-label">Transactions:</div>
          </div>
          <div class="bd-row-txs-content" id="bd-row-content-hash">
            <ul>
              {this.state.txs.map((tx, index) => {
                return (
                  <div>
                    <TX
                      number={this.state.block.number}
                      hash={JSON.parse(tx).hash}
                      fr={JSON.parse(tx).fr}
                      to={JSON.parse(tx).to}
                      value={JSON.parse(tx).value}
                    ></TX>
                  </div>
                );
              })}
            </ul>
          </div>
        </div>
      </div>
    );
  }
}

export default BlockDetails;
