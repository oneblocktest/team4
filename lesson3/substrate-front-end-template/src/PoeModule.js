import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest,setDigest]=useState('');
  const  [owner,setOwner]=useState('');
  const [block,setBlock]=useState(0);

  let fileReader;

  const bufferToDigest=()=>{
    const content=Array.from(new Uint8Array(fileReader.result)).map((b)=>b.toString(16).padStart(2,"0")).join('');
    const hash=blake2AsHex(content,256);
    setDigest(hash);
  };

  const handleFileChosen=(file)=>{
    fileReader=new FileReader();
    fileReader.onloadend=bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }



  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest,(result) => {
      setOwner(result[0].toString());
      setBlock(result[1].toNumber());
    })
      .then((unsub)=>{
        unsubscribe=unsub;
    });
      
    return () => unsubscribe && unsubscribe();
  }, [digest,api.query.poeModule]);

  return (
    <Grid.Column width={8}>
      <h1>Template Module</h1>
      <Card centered>
        <Card.Content textAlign='center'>
          <Statistic
            label='Current Value'
            value={currentValue}
          />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input
            label='New Value'
            state='newValue'
            type='number'
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Store Something'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'doSomething',
              inputParams: [formValue],
              paramFields: [true]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}

export default function poeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.something
    ? <Main {...props} /> : null);
}
