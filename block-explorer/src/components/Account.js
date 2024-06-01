import React, { Component } from "react";
import "../styles/account.css";

export class Account extends Component {
  constructor(props) {
    super(props);
    this.state = {
      hash: 0,
      balance: 0,
      nonce: 0,
    };
  }

  componentDidMount() {
    fetch("http://localhost:5000/acc/" + this.props.match.params.hash)
      .then((response) => response.json())
      .then((acc) => {
        this.setState({
          balance: acc.balance,
          nonce: acc.nonce,
          hash: this.props.match.params.hash,
        });
      });
  }

  render() {
    return (
      <div className="acc">
        <div className="acc-pubkey">{this.state.hash}</div>
        <div className="acc-balance">Balance: {this.state.balance} Eth</div>
        <div className="acc-nonce">
          Nonce: <div className="acc-nonce-number">{this.state.nonce}</div>
        </div>
      </div>
    );
  }
}

export default Account;
