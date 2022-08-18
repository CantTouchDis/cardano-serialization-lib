use std::collections::{HashMap, HashSet};
use crate::{Coin, JsError};
use super::indexes::{UtxoIndex, AssetIndex, PolicyIndex};

#[derive(Clone)]
pub(super) struct UtxosStat {
    pub(super) total_policies: usize,
    pub(super) assets_in_policy: HashMap<PolicyIndex, usize>,
    pub(super) coins_in_assets: HashMap<AssetIndex, Coin>,
    pub(super) ada_coins: Coin,
}

impl UtxosStat {
    pub(super) fn new(total_ada: &Coin, policy_to_asset: &HashMap<PolicyIndex, HashSet<AssetIndex>>,
                      amounts: &HashMap<(AssetIndex, UtxoIndex), Coin>) -> Result<Self, JsError> {
        let mut utxos_stat = UtxosStat {
            total_policies: 0,
            assets_in_policy: HashMap::new(),
            coins_in_assets: HashMap::new(),
            ada_coins: Coin::zero(),
        };
        for (policy_index, assets) in policy_to_asset {
            utxos_stat.assets_in_policy.insert(policy_index.clone(), assets.len());
        }

        for ((asset_index, utxo_index), amount) in amounts {
            if let Some(coins) = utxos_stat.coins_in_assets.get(asset_index) {
                utxos_stat.coins_in_assets.insert(asset_index.clone(), coins.checked_add(amount)?);
            } else {
                utxos_stat.coins_in_assets.insert(asset_index.clone(), amount.clone());
            }
        }

        utxos_stat.total_policies = policy_to_asset.len();
        utxos_stat.ada_coins = total_ada.clone();

        Ok(utxos_stat)
    }
}