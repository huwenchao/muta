use std::error::Error;
use std::fmt;

use core_context::{Cloneable, Context};
use core_crypto::CryptoError;
use core_types::{Hash, SignedTransaction, UnverifiedTransaction};

use crate::BoxFuture;

pub type FutTxPoolResult<T> = BoxFuture<'static, Result<T, TransactionPoolError>>;

#[derive(Clone, Debug)]
pub enum TransactionOrigin {
    Jsonrpc,
    Other,
}

impl Cloneable for TransactionOrigin {}

/// ”TransactionPool“ contains all legitimate transactions sent from other nodes
/// (P2P) or local (RPC).
pub trait TransactionPool: Sync + Send {
    /// Insert a transaction after verifying the signature and some parameters
    /// are correct.
    fn insert(
        &self,
        ctx: Context,
        untx: UnverifiedTransaction,
    ) -> FutTxPoolResult<SignedTransaction>;

    /// Filter a batch of valid transaction hashes from the transaction pool
    /// (and delete some expired transactions). Returns "count" the number
    /// of transactions if "quota_limit" does not exceed the upper limit,
    /// and returns all if the "count" number is not reached.
    /// Note: Transactions are still in the pool.
    fn package(&self, ctx: Context, count: u64, quota_limit: u64) -> FutTxPoolResult<Vec<Hash>>;

    /// Delete the specified transactions.
    fn flush(&self, ctx: Context, tx_hashes: &[Hash]) -> FutTxPoolResult<()>;

    /// Get a batch of transactions.
    fn get_batch(
        &self,
        ctx: Context,
        tx_hashes: &[Hash],
    ) -> FutTxPoolResult<Vec<SignedTransaction>>;

    /// Make sure that the transactions that specify the transactions hash are
    /// in the transaction pool. If there are transactions that do not
    /// exist, this function will request it from other nodes.

    /// NOTE: If there are no transactions in the transaction pool of this node,
    /// the function needs to obtain the missing transaction from the proposal
    /// node through P2P. and P2P needs a "p2p_session_id" to find the
    /// corresponding node. However, we don't want to pass "p2p_session_id"
    /// to this function. In the next version we will use "context" to store
    /// "p2p_session_id".
    fn ensure(&self, ctx: Context, tx_hashes: &[Hash]) -> FutTxPoolResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionPoolError {
    Dup,
    ReachLimit,
    Crypto(CryptoError),
    InvalidUntilBlock,
    QuotaNotEnough,
    NotExpected,
    TransactionNotFound,
    Internal(String),
}

impl Error for TransactionPoolError {}
impl fmt::Display for TransactionPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TransactionPoolError::Dup => "dup".to_owned(),
            TransactionPoolError::ReachLimit => "reach limit".to_owned(),
            TransactionPoolError::Crypto(ref err) => format!("signature invalid {:?}", err),
            TransactionPoolError::InvalidUntilBlock => "invalid until block".to_owned(),
            TransactionPoolError::QuotaNotEnough => "quota not enouth".to_owned(),
            TransactionPoolError::NotExpected => "not the expected transaction".to_owned(),
            TransactionPoolError::TransactionNotFound => "transaction filed is empty".to_owned(),
            TransactionPoolError::Internal(ref err) => format!("internel error {:?}", err),
        };
        write!(f, "{}", printable)
    }
}

impl From<CryptoError> for TransactionPoolError {
    fn from(err: CryptoError) -> Self {
        TransactionPoolError::Crypto(err)
    }
}
