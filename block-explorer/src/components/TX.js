//import React from "react";
import React, { Component } from "react";
import { Link } from "react-router-dom";
import "../styles/tx.css";

const HASH_LENGTH = 17;

class TX extends Component {
  render() {
    if (typeof this.props.number !== undefined) {
      var tx_link = "/tx/" + this.props.number + "/" + this.props.hash;
    } else {
      var tx_link = "/tx/" + this.props.hash;
    }

    return (
      <div>
        <div class="tx-row">
          <div class="circle">
            <div class="circle-text">TX</div>
          </div>
          <div class="col-1">
            <h4 class="tx-hash">
              <Link to={tx_link} class="block-hash">
                {String(this.props.hash).substring(0, HASH_LENGTH) + "..."}
              </Link>
            </h4>
            <h4 class="tx-time">2 min ago</h4>
          </div>
          <div class="col-2">
            <h4 class="tx-from">
              <small>FROM </small>
              <Link class="tx-from" to={"/acc/" + this.props.fr}>
                {this.props.fr.substring(0, HASH_LENGTH)}...
              </Link>
            </h4>
            <h4 class="tx-to">
              <small>TO </small>
              <Link class="tx-to" to={"/acc/" + this.props.to}>
                {this.props.to.substring(0, HASH_LENGTH)}...
              </Link>
            </h4>
          </div>
          <div class="tx-value">
            {this.props.value} ether
            <div></div>
          </div>
        </div>
      </div>
    );
  }
}

export default TX;
