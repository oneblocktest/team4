import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Message } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main(props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [receiver, setReceiver] = useState('nome');
  const [comments, setComments] = useState('nome');
  const [owner, setOwner] = useState('');
  const [block, setBlock] = useState(0);

  let fileReader;

  const bufferToDigest = () => {
    const content = Array.from(new Uint8Array(fileReader.result)).map((b) => b.toString(16).padStart(2, "0")).join('');
    const hash = blake2AsHex(content, 256);
    setDigest(hash);
  };

  const handleFileChosen = (file) => {
    fileReader = new FileReader();
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }



  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlock(result[1].toNumber());
    })
      .then((unsub) => {
        unsubscribe = unsub;
      });

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  function isClaimed() {
    return block !== 0;
  }

  return (
    <Grid.Column>
      <h1>poeModule111111111111111111111111111111111111111111</h1>
      <Form success={!!digest && !isClaimed()} warning={isClaimed()}>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='Your File'
            onChange={(e) => handleFileChosen(e.target.files[0])}
          />

          <Message success header='File Digest Unclaimed' content={digest} />
          <Message
            warning
            header='File Digest Claimed'
            list={[digest, `Owner: ${owner}`, `Block: ${block}`, `memo: ${comments}`]}
          />
        </Form.Field>
        <Form.Field>
          <Input
            label='memo'
            state='comments'
            type='string'
            onChange={(_, { value }) => setComments(value)}
          />
        </Form.Field>
        
        <Form.Field>
          <Input
            label='Claim Receiver'
            state='receiver'
            type='string'
            onChange={(_, { value }) => setReceiver(value)}
          />
        </Form.Field>
        <Form.Field>

        <TxButton
            accountPair={accountPair}
            label={`Create Claim memo`}
            setStatus={setStatus}
            type="SIGNED-TX"
            disabled={isClaimed() || !digest || !comments}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaimComment',
              inputParams: [digest,comments],
              paramFields: [true]

            }}
          />
          <TxButton
            accountPair={accountPair}
            label={`Create Claim`}
            setStatus={setStatus}
            type="SIGNED-TX"
            disabled={isClaimed() || !digest}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest],
              paramFields: [true]

            }}
          />

          <TxButton
            accountPair={accountPair}
            label={`Revoke Claim`}
            setStatus={setStatus}
            type="SIGNED-TX"
            disabled={!isClaimed() || owner !== accountPair.address}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]

            }}
          />
          <TxButton
            accountPair={accountPair}
            label={`Transfer Claim`}
            setStatus={setStatus}
            type="SIGNED-TX"
            disabled={!isClaimed() || !receiver}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, receiver],
              paramFields: [true]

            }}
          />
        </Form.Field>
      </Form>
    </Grid.Column>

  );
}

export default function poeModule(props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
