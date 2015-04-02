/*  Copyright 2015 MaidSafe.net limited
    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").
    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses
    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.
    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe
    Software.                                                                 */

#![allow(dead_code)]

pub struct Entry {
  name: Vec<u8>,
  data: Vec<u8>
}

pub struct ChunkStore {
  entries: Vec<Entry>,
  max_disk_usage: usize,
  current_disk_usage: usize,
}

impl ChunkStore {
  pub fn new() -> ChunkStore {
    ChunkStore {
      entries: Vec::new(),
      max_disk_usage: 0,
      current_disk_usage: 0,
    }
  }

  pub fn put(&mut self, name: Vec<u8>, value: Vec<u8>) {


        self.current_disk_usage += value.len();

        self.entries.push(Entry {
          name: name,
          data: value,
        });
  }

  pub fn delete(&mut self, name: Vec<u8>) {
    let mut size_removed = 0usize;

    for i in 0..self.entries.len() {
      if self.entries[i].name == name {
        size_removed = self.entries[i].data.len();
        self.entries.remove(i);
        break;
      }
    }

    self.current_disk_usage -= size_removed;
  }

  pub fn get(&self, name: Vec<u8>) -> Vec<u8> {
      unimplemented!();
  // something like this should be used and return a result
  //  self.entries.iter().take(|x| { x == name });

  }

  pub fn max_disk_usage(&self) -> usize {
    self.max_disk_usage
  }

  pub fn current_disk_usage(&self) -> usize {
    self.current_disk_usage
  }

  pub fn set_max_disk_usage(&mut self, new_max: usize) {
    assert!(self.current_disk_usage < new_max);
    self.max_disk_usage = new_max;
  }

  pub fn has_chunk(&self, name: Vec<u8>) -> bool {
    for entry in self.entries.iter() {
      if entry.name == name {
        return true;
      }
    }
    false
  }

  pub fn names(&self) -> Vec<Vec<u8>> {
    let mut name_vec: Vec<Vec<u8>> = Vec::new();
    for it in self.entries.iter() {
      name_vec.push(it.name.clone());
    }

    name_vec
  }
// FIXME: Unused
  // fn has_disk_space(&self, required_space: usize) -> bool {
  //   self.current_disk_usage + required_space <= self.max_disk_usage
  // }
}
