use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResumeOutput {
  pub etchings: Vec<batch::Output>,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
  let  etchings = Vec::new();

    for (rune, entry) in wallet.pending_etchings()? {
// //       if wallet.is_mature(&entry.commit)? {
// //         etchings.push(wallet.send_etching(rune, &entry)?);
// //       }
        println!("Commit: {} etching: {}, rune: {}",
                 hex::encode(&entry.commit.txid()),
                 hex::encode(&entry.reveal.txid()),
                rune
        );
// //         println!("{}",hex::encode(&signed_reveal_tx));
    }

//     if wallet.pending_etchings()?.is_empty() {
//       break;
//     }
//
//     if !wallet.integration_test() {
//       thread::sleep(Duration::from_secs(5));
//     }

   Ok(Some(Box::new(ResumeOutput { etchings }) as Box<dyn Output>))
}
