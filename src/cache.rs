// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use lru_time_cache::LruCache;
use routing::{ImmutableData, Request, Response, XorName};
use routing::Cache as RoutingCache;
use std::cell::RefCell;
use std::time::Duration;

const CACHE_CAPACITY: usize = 1000;
const CACHE_EXPIRY_DURATION_SECS: u64 = 60 * 60;

pub struct Cache {
    store: RefCell<LruCache<XorName, ImmutableData>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            store: RefCell::new(LruCache::with_expiry_duration_and_capacity(
                                    Duration::from_secs(CACHE_EXPIRY_DURATION_SECS),
                                    CACHE_CAPACITY)),
        }
    }
}

impl RoutingCache for Cache {
    fn get(&self, request: &Request) -> Option<Response> {
        if let Request::GetIData { name, msg_id } = *request {
            self.store
                .borrow_mut()
                .get(&name)
                .map(|data| {
                         Response::GetIData {
                             res: Ok(data.clone()),
                             msg_id: msg_id,
                         }
                     })
        } else {
            None
        }
    }

    fn put(&self, response: Response) {
        if let Response::GetIData { res: Ok(data), .. } = response {
            let _ = self.store.borrow_mut().insert(*data.name(), data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cache;
    use routing::{ImmutableData, MessageId, Request, Response};
    use routing::Cache as RoutingCache;

    #[test]
    fn put_and_get() {
        let cache = Cache::new();

        let data = "hello world".bytes().collect();
        let data = ImmutableData::new(data);
        let response_message_id = MessageId::new();

        let response = Response::GetIData {
            res: Ok(data.clone()),
            msg_id: response_message_id,
        };
        cache.put(response);

        let request_message_id = MessageId::new();
        let request = Request::GetIData {
            name: *data.name(),
            msg_id: request_message_id,
        };

        match cache.get(&request) {
            Some(Response::GetIData {
                     res: Ok(cached_data),
                     msg_id: cached_message_id,
                 }) => {
                assert_eq!(cached_data, data);
                assert_eq!(cached_message_id, request_message_id);
            }
            _ => panic!("unexpected cached value"),
        }
    }
}
