// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

#[allow(dead_code)]
/// Reader is used to read contents of a File. It can read in chunks if the file happens to be very
/// large
pub struct Reader {
    file: ::file::File,
    self_encryptor: ::self_encryption::SelfEncryptor<::maidsafe_client::SelfEncryptionStorage>,
    client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>,
}

impl Reader {
    /// Create a new instance of Reader
    pub fn new(file: ::file::File,
               client: ::std::sync::Arc<::std::sync::Mutex<::maidsafe_client::client::Client>>) -> Reader {        
        Reader {
            file: file.clone(),
            self_encryptor: ::self_encryption::SelfEncryptor::new(::maidsafe_client::SelfEncryptionStorage::new(client.clone()), file.get_datamap().clone()),
            client: client,
        }
    }

    /// Returns the total size of the file/blob
    pub fn size(&self) -> u64 {
        self.self_encryptor.len()
    }

    /// Read data from file/blob
    pub fn read(&mut self,  position: u64, length: u64) -> Result<Vec<u8>, String> {
        if (position + length) > self.size() {
            return Err("Invalid range specified".to_string());
        }
        Ok(self.self_encryptor.read(position, length))
    }
}