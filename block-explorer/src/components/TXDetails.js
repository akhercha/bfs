//import React from "react";
import React, { Component } from "react";
import question from "../svgs/question.svg";
import ok from "../svgs/checkOk.svg";
import "../styles/txdetails.css";

import { BsFillExclamationOctagonFill as FailedIcon } from "react-icons/bs";
import { Link } from "react-router-dom";

class TXDetails extends Component {
  constructor(props) {
    super(props);
    this.state = {
      tx: [],
    };
  }

  componentDidMount() {
    var blockNumber = this.props.match.params.block_number;

    if (blockNumber) {
      var tx_link =
        "http://localhost:5000/tx/" +
        blockNumber +
        "/" +
        this.props.match.params.hash;
    }

    if (blockNumber == "undefined") {
      var tx_link =
        "http://localhost:5000/tx_pool/" + this.props.match.params.hash;
    }

    console.log(tx_link);
    fetch(tx_link)
      .then((response) => response.json())
      .then((tx) => {
        console.log(JSON.parse(tx.tx));
        this.setState({ tx: JSON.parse(tx.tx) });
      });
  }

  render() {
    var isSigned = this.state.tx.signed;
    return (
      <div class="main">
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="label">Transactions Hash:</h5>
        </div>
        <div class="hash">{this.state.tx.hash}</div>
        <div class="hl"></div>
        <div class="txd-row status-row">
          <img class="question" src={question} />
          <h5 class="label">Signed:</h5>
          {isSigned ? (
            <div class="bd-isSigned-true">
              <img class="ok" src={ok} />
              <div class="status">Yes</div>
            </div>
          ) : (
            <div class="bd-isSigned-false">
              <FailedIcon
                class="bd-isSigned-false-icon"
                style={{ color: "rgb(255, 71, 26)", size: "50px" }}
              />
              <div class="bd-isSigned-false-label">No</div>
            </div>
          )}
        </div>
        <div class="hl"></div>
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="label">Number:</h5>
        </div>
        <h5 class="block-number">122234</h5>
        <div class="hl"></div>
        <div class="txd-row time-row">
          <img class="question" src={question} />
          <h5 class="label">Timestamp:</h5>
        </div>
        <div class="time">{this.state.tx.time}</div>
        <div class="hl"></div>
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="label">From:</h5>
        </div>
        <div class="from">
          <Link class="from" to={"/acc/" + this.state.tx.fr}>
            {this.state.tx.fr}
          </Link>
        </div>
        <div class="hl"></div>
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="label">To:</h5>
        </div>
        <div class="from">
          <Link class="from" to={"/acc/" + this.state.tx.to}>
            {this.state.tx.to}
          </Link>
        </div>

        <div class="hl"></div>
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="value">Value:</h5>
        </div>
        <div class="value">{this.state.tx.value} ether</div>

        <div class="hl"></div>
        <div class="txd-row">
          <img class="question" src={question} />
          <h5 class="value">Nonce:</h5>
          <div class="nonce">{this.state.tx.nonce}</div>
        </div>
      </div>
    );
  }
}

export default TXDetails;
