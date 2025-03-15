// @generated
impl serde::Serialize for AccountCleared {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.AccountCleared", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccountCleared {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "recipient"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Recipient,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "recipient" => Ok(GeneratedField::Recipient),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountCleared;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.AccountCleared")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccountCleared, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut recipient__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AccountCleared {
                    address: address__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.AccountCleared",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AccountRegistered {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.fallback.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.AccountRegistered", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.fallback.is_empty() {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccountRegistered {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "channel", "recipient", "fallback"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Channel,
            Recipient,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "channel" => Ok(GeneratedField::Channel),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountRegistered;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.AccountRegistered")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccountRegistered, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut channel__ = None;
                let mut recipient__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AccountRegistered {
                    address: address__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.AccountRegistered",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for AllowedDenomsConfigured {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_denoms.is_empty() {
            len += 1;
        }
        if !self.current_denoms.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.AllowedDenomsConfigured", len)?;
        if !self.previous_denoms.is_empty() {
            struct_ser.serialize_field("previousDenoms", &self.previous_denoms)?;
        }
        if !self.current_denoms.is_empty() {
            struct_ser.serialize_field("currentDenoms", &self.current_denoms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedDenomsConfigured {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_denoms",
            "previousDenoms",
            "current_denoms",
            "currentDenoms",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousDenoms,
            CurrentDenoms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "previousDenoms" | "previous_denoms" => {
                                Ok(GeneratedField::PreviousDenoms)
                            }
                            "currentDenoms" | "current_denoms" => Ok(GeneratedField::CurrentDenoms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedDenomsConfigured;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.AllowedDenomsConfigured")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AllowedDenomsConfigured, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_denoms__ = None;
                let mut current_denoms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousDenoms => {
                            if previous_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousDenoms"));
                            }
                            previous_denoms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrentDenoms => {
                            if current_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentDenoms"));
                            }
                            current_denoms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllowedDenomsConfigured {
                    previous_denoms: previous_denoms__.unwrap_or_default(),
                    current_denoms: current_denoms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.AllowedDenomsConfigured",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ForwardingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_account.is_some() {
            len += 1;
        }
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if !self.fallback.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.ForwardingAccount", len)?;
        if let Some(v) = self.base_account.as_ref() {
            struct_ser.serialize_field("baseAccount", v)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if !self.fallback.is_empty() {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_account",
            "baseAccount",
            "channel",
            "recipient",
            "created_at",
            "createdAt",
            "fallback",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseAccount,
            Channel,
            Recipient,
            CreatedAt,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseAccount" | "base_account" => Ok(GeneratedField::BaseAccount),
                            "channel" => Ok(GeneratedField::Channel),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.ForwardingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForwardingAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_account__ = None;
                let mut channel__ = None;
                let mut recipient__ = None;
                let mut created_at__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseAccount => {
                            if base_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAccount"));
                            }
                            base_account__ = map_.next_value()?;
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ForwardingAccount {
                    base_account: base_account__,
                    channel: channel__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.ForwardingAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ForwardingPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.ForwardingPubKey", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardingPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardingPubKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.ForwardingPubKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForwardingPubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ForwardingPubKey {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.ForwardingPubKey",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_denoms.is_empty() {
            len += 1;
        }
        if !self.num_of_accounts.is_empty() {
            len += 1;
        }
        if !self.num_of_forwards.is_empty() {
            len += 1;
        }
        if !self.total_forwarded.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.GenesisState", len)?;
        if !self.allowed_denoms.is_empty() {
            struct_ser.serialize_field("allowedDenoms", &self.allowed_denoms)?;
        }
        if !self.num_of_accounts.is_empty() {
            let v: std::collections::HashMap<_, _> = self
                .num_of_accounts
                .iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            struct_ser.serialize_field("numOfAccounts", &v)?;
        }
        if !self.num_of_forwards.is_empty() {
            let v: std::collections::HashMap<_, _> = self
                .num_of_forwards
                .iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            struct_ser.serialize_field("numOfForwards", &v)?;
        }
        if !self.total_forwarded.is_empty() {
            struct_ser.serialize_field("totalForwarded", &self.total_forwarded)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_denoms",
            "allowedDenoms",
            "num_of_accounts",
            "numOfAccounts",
            "num_of_forwards",
            "numOfForwards",
            "total_forwarded",
            "totalForwarded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedDenoms,
            NumOfAccounts,
            NumOfForwards,
            TotalForwarded,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowedDenoms" | "allowed_denoms" => Ok(GeneratedField::AllowedDenoms),
                            "numOfAccounts" | "num_of_accounts" => {
                                Ok(GeneratedField::NumOfAccounts)
                            }
                            "numOfForwards" | "num_of_forwards" => {
                                Ok(GeneratedField::NumOfForwards)
                            }
                            "totalForwarded" | "total_forwarded" => {
                                Ok(GeneratedField::TotalForwarded)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allowed_denoms__ = None;
                let mut num_of_accounts__ = None;
                let mut num_of_forwards__ = None;
                let mut total_forwarded__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedDenoms => {
                            if allowed_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedDenoms"));
                            }
                            allowed_denoms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumOfAccounts => {
                            if num_of_accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfAccounts"));
                            }
                            num_of_accounts__ = Some(
                                map_.next_value::<std::collections::HashMap<
                                    _,
                                    ::pbjson::private::NumberDeserialize<u64>,
                                >>()?
                                .into_iter()
                                .map(|(k, v)| (k, v.0))
                                .collect(),
                            );
                        }
                        GeneratedField::NumOfForwards => {
                            if num_of_forwards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfForwards"));
                            }
                            num_of_forwards__ = Some(
                                map_.next_value::<std::collections::HashMap<
                                    _,
                                    ::pbjson::private::NumberDeserialize<u64>,
                                >>()?
                                .into_iter()
                                .map(|(k, v)| (k, v.0))
                                .collect(),
                            );
                        }
                        GeneratedField::TotalForwarded => {
                            if total_forwarded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalForwarded"));
                            }
                            total_forwarded__ =
                                Some(map_.next_value::<std::collections::HashMap<_, _>>()?);
                        }
                    }
                }
                Ok(GenesisState {
                    allowed_denoms: allowed_denoms__.unwrap_or_default(),
                    num_of_accounts: num_of_accounts__.unwrap_or_default(),
                    num_of_forwards: num_of_forwards__.unwrap_or_default(),
                    total_forwarded: total_forwarded__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgClearAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.fallback {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgClearAccount", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.fallback {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClearAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "address", "fallback"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Address,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signer" => Ok(GeneratedField::Signer),
                            "address" => Ok(GeneratedField::Address),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClearAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgClearAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgClearAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut address__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgClearAccount {
                    signer: signer__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgClearAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgClearAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgClearAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClearAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClearAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgClearAccountResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgClearAccountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgClearAccountResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgClearAccountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRegisterAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.fallback.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgRegisterAccount", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.fallback.is_empty() {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "recipient", "channel", "fallback"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Recipient,
            Channel,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signer" => Ok(GeneratedField::Signer),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "channel" => Ok(GeneratedField::Channel),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgRegisterAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRegisterAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut recipient__ = None;
                let mut channel__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRegisterAccount {
                    signer: signer__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgRegisterAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRegisterAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgRegisterAccountResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgRegisterAccountResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRegisterAccountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRegisterAccountResponse {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgRegisterAccountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetAllowedDenoms {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.denoms.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgSetAllowedDenoms", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.denoms.is_empty() {
            struct_ser.serialize_field("denoms", &self.denoms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetAllowedDenoms {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "denoms"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Denoms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signer" => Ok(GeneratedField::Signer),
                            "denoms" => Ok(GeneratedField::Denoms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetAllowedDenoms;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgSetAllowedDenoms")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetAllowedDenoms, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut denoms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denoms => {
                            if denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denoms"));
                            }
                            denoms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetAllowedDenoms {
                    signer: signer__.unwrap_or_default(),
                    denoms: denoms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgSetAllowedDenoms",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetAllowedDenomsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.MsgSetAllowedDenomsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetAllowedDenomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetAllowedDenomsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.MsgSetAllowedDenomsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetAllowedDenomsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetAllowedDenomsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.MsgSetAllowedDenomsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.fallback.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryAddress", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.fallback.is_empty() {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channel", "recipient", "fallback"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            Recipient,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "channel" => Ok(GeneratedField::Channel),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut recipient__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAddress {
                    channel: channel__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.exists {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryAddressResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.exists {
            struct_ser.serialize_field("exists", &self.exists)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "exists"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Exists,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "exists" => Ok(GeneratedField::Exists),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryAddressResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut exists__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exists => {
                            if exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            exists__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAddressResponse {
                    address: address__.unwrap_or_default(),
                    exists: exists__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryDenoms {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.forwarding.v1.QueryDenoms", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDenoms {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDenoms;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryDenoms")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryDenoms, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryDenoms {})
            }
        }
        deserializer.deserialize_struct("noble.forwarding.v1.QueryDenoms", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryDenomsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_denoms.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryDenomsResponse", len)?;
        if !self.allowed_denoms.is_empty() {
            struct_ser.serialize_field("allowedDenoms", &self.allowed_denoms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDenomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["allowed_denoms", "allowedDenoms"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedDenoms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowedDenoms" | "allowed_denoms" => Ok(GeneratedField::AllowedDenoms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDenomsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryDenomsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryDenomsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allowed_denoms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedDenoms => {
                            if allowed_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedDenoms"));
                            }
                            allowed_denoms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDenomsResponse {
                    allowed_denoms: allowed_denoms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryDenomsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.forwarding.v1.QueryStats", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStats, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryStats {})
            }
        }
        deserializer.deserialize_struct("noble.forwarding.v1.QueryStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryStatsByChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryStatsByChannel", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStatsByChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channel"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStatsByChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryStatsByChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStatsByChannel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStatsByChannel {
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryStatsByChannel",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryStatsByChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_of_accounts != 0 {
            len += 1;
        }
        if self.num_of_forwards != 0 {
            len += 1;
        }
        if !self.total_forwarded.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryStatsByChannelResponse", len)?;
        if self.num_of_accounts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "numOfAccounts",
                ToString::to_string(&self.num_of_accounts).as_str(),
            )?;
        }
        if self.num_of_forwards != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "numOfForwards",
                ToString::to_string(&self.num_of_forwards).as_str(),
            )?;
        }
        if !self.total_forwarded.is_empty() {
            struct_ser.serialize_field("totalForwarded", &self.total_forwarded)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStatsByChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_of_accounts",
            "numOfAccounts",
            "num_of_forwards",
            "numOfForwards",
            "total_forwarded",
            "totalForwarded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumOfAccounts,
            NumOfForwards,
            TotalForwarded,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "numOfAccounts" | "num_of_accounts" => {
                                Ok(GeneratedField::NumOfAccounts)
                            }
                            "numOfForwards" | "num_of_forwards" => {
                                Ok(GeneratedField::NumOfForwards)
                            }
                            "totalForwarded" | "total_forwarded" => {
                                Ok(GeneratedField::TotalForwarded)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStatsByChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryStatsByChannelResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryStatsByChannelResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut num_of_accounts__ = None;
                let mut num_of_forwards__ = None;
                let mut total_forwarded__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumOfAccounts => {
                            if num_of_accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfAccounts"));
                            }
                            num_of_accounts__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NumOfForwards => {
                            if num_of_forwards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfForwards"));
                            }
                            num_of_forwards__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalForwarded => {
                            if total_forwarded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalForwarded"));
                            }
                            total_forwarded__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStatsByChannelResponse {
                    num_of_accounts: num_of_accounts__.unwrap_or_default(),
                    num_of_forwards: num_of_forwards__.unwrap_or_default(),
                    total_forwarded: total_forwarded__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryStatsByChannelResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryStatsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stats.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.QueryStatsResponse", len)?;
        if !self.stats.is_empty() {
            struct_ser.serialize_field("stats", &self.stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStatsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["stats"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stats,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stats" => Ok(GeneratedField::Stats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.QueryStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStatsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = Some(map_.next_value::<std::collections::HashMap<_, _>>()?);
                        }
                    }
                }
                Ok(QueryStatsResponse {
                    stats: stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.QueryStatsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RegisterAccountData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.fallback.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.RegisterAccountData", len)?;
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.fallback.is_empty() {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterAccountData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["recipient", "channel", "fallback"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipient,
            Channel,
            Fallback,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "recipient" => Ok(GeneratedField::Recipient),
                            "channel" => Ok(GeneratedField::Channel),
                            "fallback" => Ok(GeneratedField::Fallback),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterAccountData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.RegisterAccountData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterAccountData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut recipient__ = None;
                let mut channel__ = None;
                let mut fallback__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisterAccountData {
                    recipient: recipient__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.RegisterAccountData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RegisterAccountMemo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.noble.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.forwarding.v1.RegisterAccountMemo", len)?;
        if let Some(v) = self.noble.as_ref() {
            struct_ser.serialize_field("noble", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterAccountMemo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["noble"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Noble,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "noble" => Ok(GeneratedField::Noble),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterAccountMemo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.RegisterAccountMemo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterAccountMemo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut noble__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Noble => {
                            if noble__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noble"));
                            }
                            noble__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisterAccountMemo { noble: noble__ })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.RegisterAccountMemo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for register_account_memo::RegisterAccountDataWrapper {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.forwarding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "noble.forwarding.v1.RegisterAccountMemo.RegisterAccountDataWrapper",
            len,
        )?;
        if let Some(v) = self.forwarding.as_ref() {
            struct_ser.serialize_field("forwarding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for register_account_memo::RegisterAccountDataWrapper {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["forwarding"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Forwarding,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "forwarding" => Ok(GeneratedField::Forwarding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = register_account_memo::RegisterAccountDataWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct noble.forwarding.v1.RegisterAccountMemo.RegisterAccountDataWrapper",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<register_account_memo::RegisterAccountDataWrapper, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut forwarding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Forwarding => {
                            if forwarding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwarding"));
                            }
                            forwarding__ = map_.next_value()?;
                        }
                    }
                }
                Ok(register_account_memo::RegisterAccountDataWrapper {
                    forwarding: forwarding__,
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.forwarding.v1.RegisterAccountMemo.RegisterAccountDataWrapper",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Stats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.num_of_accounts != 0 {
            len += 1;
        }
        if self.num_of_forwards != 0 {
            len += 1;
        }
        if !self.total_forwarded.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.forwarding.v1.Stats", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.num_of_accounts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "numOfAccounts",
                ToString::to_string(&self.num_of_accounts).as_str(),
            )?;
        }
        if self.num_of_forwards != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "numOfForwards",
                ToString::to_string(&self.num_of_forwards).as_str(),
            )?;
        }
        if !self.total_forwarded.is_empty() {
            struct_ser.serialize_field("totalForwarded", &self.total_forwarded)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Stats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "num_of_accounts",
            "numOfAccounts",
            "num_of_forwards",
            "numOfForwards",
            "total_forwarded",
            "totalForwarded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            NumOfAccounts,
            NumOfForwards,
            TotalForwarded,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "numOfAccounts" | "num_of_accounts" => {
                                Ok(GeneratedField::NumOfAccounts)
                            }
                            "numOfForwards" | "num_of_forwards" => {
                                Ok(GeneratedField::NumOfForwards)
                            }
                            "totalForwarded" | "total_forwarded" => {
                                Ok(GeneratedField::TotalForwarded)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Stats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.forwarding.v1.Stats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Stats, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut num_of_accounts__ = None;
                let mut num_of_forwards__ = None;
                let mut total_forwarded__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumOfAccounts => {
                            if num_of_accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfAccounts"));
                            }
                            num_of_accounts__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NumOfForwards => {
                            if num_of_forwards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfForwards"));
                            }
                            num_of_forwards__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalForwarded => {
                            if total_forwarded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalForwarded"));
                            }
                            total_forwarded__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Stats {
                    chain_id: chain_id__.unwrap_or_default(),
                    num_of_accounts: num_of_accounts__.unwrap_or_default(),
                    num_of_forwards: num_of_forwards__.unwrap_or_default(),
                    total_forwarded: total_forwarded__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.forwarding.v1.Stats", FIELDS, GeneratedVisitor)
    }
}
