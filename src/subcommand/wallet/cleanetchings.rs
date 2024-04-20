use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResumeOutput {
  pub etchings: Vec<batch::Output>,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
  let etchings = Vec::new();

    for (rune, entry) in wallet.pending_etchings()? {
        println!("Commit: {} Etching {}",
                 hex::encode(&entry.commit.txid()),
                 hex::encode(&entry.reveal.txid()));
        let _ = wallet.clear_etching(rune);
    }


   Ok(Some(Box::new(ResumeOutput { etchings }) as Box<dyn Output>))
}
