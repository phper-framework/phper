// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpClientError, request::RequestBuilderClass};
use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, StateClass, Visibility},
    functions::Argument,
};
use reqwest::blocking::{Client, ClientBuilder};
use std::{convert::Infallible, mem::take, time::Duration};

pub type ClientBuilderClass = StateClass<ClientBuilder>;

pub type ClientClass = StateClass<Option<Client>>;

const HTTP_CLIENT_BUILDER_CLASS_NAME: &str = "HttpClient\\HttpClientBuilder";

const HTTP_CLIENT_CLASS_NAME: &str = "HttpClient\\HttpClient";

pub fn make_client_builder_class(client_class: ClientClass) -> ClassEntity<ClientBuilder> {
    // `new_with_default_state_constructor` means initialize the state of
    // `ClientBuilder` as `Default::default`.
    let mut class = ClassEntity::new_with_default_state_constructor(HTTP_CLIENT_BUILDER_CLASS_NAME);

    // Inner call the `ClientBuilder::timeout`.
    class
        .add_method("timeout", Visibility::Public, |this, arguments| {
            let ms = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.timeout(Duration::from_millis(ms as u64));
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::new("ms"));

    // Inner call the `ClientBuilder::cookie_store`.
    class
        .add_method("cookie_store", Visibility::Public, |this, arguments| {
            let enable = arguments[0].expect_bool()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.cookie_store(enable);
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::new("enable"));

    // Inner call the `ClientBuilder::build`, and wrap the result `Client` in
    // Object.
    class.add_method("build", Visibility::Public, move |this, _arguments| {
        let state = take(this.as_mut_state());
        let client = ClientBuilder::build(state).map_err(HttpClientError::Reqwest)?;
        let mut object = client_class.init_object()?;
        *object.as_mut_state() = Some(client);
        Ok::<_, phper::Error>(object)
    });

    class
}

pub fn make_client_class(
    request_builder_class: RequestBuilderClass,
) -> ClassEntity<Option<Client>> {
    let mut class =
        ClassEntity::<Option<Client>>::new_with_default_state_constructor(HTTP_CLIENT_CLASS_NAME);

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    let request_build_class_ = request_builder_class.clone();
    class
        .add_method("get", Visibility::Public, move |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.get(url);
            let mut object = request_build_class_.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::new("url"));

    class
        .add_method("post", Visibility::Public, move |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.post(url);
            let mut object = request_builder_class.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::new("url"));

    class
}
