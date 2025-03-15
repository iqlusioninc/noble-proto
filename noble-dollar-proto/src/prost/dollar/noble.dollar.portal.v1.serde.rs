// @generated
impl serde::Serialize for BridgingPath {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.destination_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.BridgingPath", len)?;
        if self.destination_chain_id != 0 {
            struct_ser.serialize_field("destinationChainId", &self.destination_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BridgingPath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "destination_chain_id",
            "destinationChainId",
            "destination_token",
            "destinationToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationChainId,
            DestinationToken,
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
                            "destinationChainId" | "destination_chain_id" => {
                                Ok(GeneratedField::DestinationChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
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
            type Value = BridgingPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.BridgingPath")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BridgingPath, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut destination_chain_id__ = None;
                let mut destination_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DestinationChainId => {
                            if destination_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationChainId",
                                ));
                            }
                            destination_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BridgingPath {
                    destination_chain_id: destination_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.BridgingPath",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BridgingPathSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.destination_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        if self.supported {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.BridgingPathSet", len)?;
        if self.destination_chain_id != 0 {
            struct_ser.serialize_field("destinationChainId", &self.destination_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        if self.supported {
            struct_ser.serialize_field("supported", &self.supported)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BridgingPathSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "destination_chain_id",
            "destinationChainId",
            "destination_token",
            "destinationToken",
            "supported",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationChainId,
            DestinationToken,
            Supported,
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
                            "destinationChainId" | "destination_chain_id" => {
                                Ok(GeneratedField::DestinationChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
                            }
                            "supported" => Ok(GeneratedField::Supported),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BridgingPathSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.BridgingPathSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BridgingPathSet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut destination_chain_id__ = None;
                let mut destination_token__ = None;
                let mut supported__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DestinationChainId => {
                            if destination_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationChainId",
                                ));
                            }
                            destination_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Supported => {
                            if supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supported"));
                            }
                            supported__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BridgingPathSet {
                    destination_chain_id: destination_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                    supported: supported__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.BridgingPathSet",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Delivered {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vaa.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.Delivered", len)?;
        if !self.vaa.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("vaa", pbjson::private::base64::encode(&self.vaa).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Delivered {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["vaa"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vaa,
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
                            "vaa" => Ok(GeneratedField::Vaa),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Delivered;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.Delivered")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Delivered, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut vaa__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vaa => {
                            if vaa__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaa"));
                            }
                            vaa__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Delivered {
                    vaa: vaa__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.Delivered",
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
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.paused {
            len += 1;
        }
        if !self.peers.is_empty() {
            len += 1;
        }
        if !self.bridging_paths.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.GenesisState", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        if !self.peers.is_empty() {
            struct_ser.serialize_field("peers", &self.peers)?;
        }
        if !self.bridging_paths.is_empty() {
            struct_ser.serialize_field("bridgingPaths", &self.bridging_paths)?;
        }
        if self.nonce != 0 {
            struct_ser.serialize_field("nonce", &self.nonce)?;
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
            "owner",
            "paused",
            "peers",
            "bridging_paths",
            "bridgingPaths",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            Paused,
            Peers,
            BridgingPaths,
            Nonce,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "paused" => Ok(GeneratedField::Paused),
                            "peers" => Ok(GeneratedField::Peers),
                            "bridgingPaths" | "bridging_paths" => Ok(GeneratedField::BridgingPaths),
                            "nonce" => Ok(GeneratedField::Nonce),
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
                formatter.write_str("struct noble.dollar.portal.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut paused__ = None;
                let mut peers__ = None;
                let mut bridging_paths__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Peers => {
                            if peers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peers"));
                            }
                            peers__ = Some(
                                map_.next_value::<std::collections::HashMap<
                                    ::pbjson::private::NumberDeserialize<u32>,
                                    _,
                                >>()?
                                .into_iter()
                                .map(|(k, v)| (k.0, v))
                                .collect(),
                            );
                        }
                        GeneratedField::BridgingPaths => {
                            if bridging_paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgingPaths"));
                            }
                            bridging_paths__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(GenesisState {
                    owner: owner__.unwrap_or_default(),
                    paused: paused__.unwrap_or_default(),
                    peers: peers__.unwrap_or_default(),
                    bridging_paths: bridging_paths__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MTokenReceived {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.message_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MTokenReceived", len)?;
        if self.source_chain_id != 0 {
            struct_ser.serialize_field("sourceChainId", &self.source_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        if !self.sender.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "sender",
                pbjson::private::base64::encode(&self.sender).as_str(),
            )?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.message_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageId",
                pbjson::private::base64::encode(&self.message_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MTokenReceived {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_chain_id",
            "sourceChainId",
            "destination_token",
            "destinationToken",
            "sender",
            "recipient",
            "amount",
            "index",
            "message_id",
            "messageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceChainId,
            DestinationToken,
            Sender,
            Recipient,
            Amount,
            Index,
            MessageId,
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
                            "sourceChainId" | "source_chain_id" => {
                                Ok(GeneratedField::SourceChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
                            }
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "index" => Ok(GeneratedField::Index),
                            "messageId" | "message_id" => Ok(GeneratedField::MessageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MTokenReceived;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MTokenReceived")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MTokenReceived, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_chain_id__ = None;
                let mut destination_token__ = None;
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut amount__ = None;
                let mut index__ = None;
                let mut message_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceChainId => {
                            if source_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceChainId"));
                            }
                            source_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MessageId => {
                            if message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageId"));
                            }
                            message_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MTokenReceived {
                    source_chain_id: source_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    message_id: message_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MTokenReceived",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeliver {
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
        if !self.vaa.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgDeliver", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.vaa.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("vaa", pbjson::private::base64::encode(&self.vaa).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeliver {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "vaa"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Vaa,
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
                            "vaa" => Ok(GeneratedField::Vaa),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeliver;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgDeliver")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDeliver, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut vaa__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vaa => {
                            if vaa__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaa"));
                            }
                            vaa__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgDeliver {
                    signer: signer__.unwrap_or_default(),
                    vaa: vaa__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgDeliver",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeliverInjection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vaa.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgDeliverInjection", len)?;
        if !self.vaa.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("vaa", pbjson::private::base64::encode(&self.vaa).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeliverInjection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["vaa"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vaa,
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
                            "vaa" => Ok(GeneratedField::Vaa),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeliverInjection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgDeliverInjection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDeliverInjection, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut vaa__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vaa => {
                            if vaa__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaa"));
                            }
                            vaa__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgDeliverInjection {
                    vaa: vaa__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgDeliverInjection",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeliverResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgDeliverResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeliverResponse {
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
            type Value = MsgDeliverResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgDeliverResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDeliverResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDeliverResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgDeliverResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetBridgingPath {
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
        if self.destination_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        if self.supported {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgSetBridgingPath", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.destination_chain_id != 0 {
            struct_ser.serialize_field("destinationChainId", &self.destination_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        if self.supported {
            struct_ser.serialize_field("supported", &self.supported)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetBridgingPath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "destination_chain_id",
            "destinationChainId",
            "destination_token",
            "destinationToken",
            "supported",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            DestinationChainId,
            DestinationToken,
            Supported,
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
                            "destinationChainId" | "destination_chain_id" => {
                                Ok(GeneratedField::DestinationChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
                            }
                            "supported" => Ok(GeneratedField::Supported),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetBridgingPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetBridgingPath")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetBridgingPath, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut destination_chain_id__ = None;
                let mut destination_token__ = None;
                let mut supported__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationChainId => {
                            if destination_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationChainId",
                                ));
                            }
                            destination_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Supported => {
                            if supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supported"));
                            }
                            supported__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetBridgingPath {
                    signer: signer__.unwrap_or_default(),
                    destination_chain_id: destination_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                    supported: supported__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetBridgingPath",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetBridgingPathResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("noble.dollar.portal.v1.MsgSetBridgingPathResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetBridgingPathResponse {
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
            type Value = MsgSetBridgingPathResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetBridgingPathResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetBridgingPathResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetBridgingPathResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetBridgingPathResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPausedState {
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
        if self.paused {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgSetPausedState", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPausedState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "paused"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Paused,
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
                            "paused" => Ok(GeneratedField::Paused),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetPausedState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetPausedState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetPausedState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut paused__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetPausedState {
                    signer: signer__.unwrap_or_default(),
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetPausedState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPausedStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgSetPausedStateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPausedStateResponse {
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
            type Value = MsgSetPausedStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetPausedStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetPausedStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetPausedStateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetPausedStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPeer {
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
        if self.chain != 0 {
            len += 1;
        }
        if !self.transceiver.is_empty() {
            len += 1;
        }
        if !self.manager.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgSetPeer", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.chain != 0 {
            struct_ser.serialize_field("chain", &self.chain)?;
        }
        if !self.transceiver.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "transceiver",
                pbjson::private::base64::encode(&self.transceiver).as_str(),
            )?;
        }
        if !self.manager.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "manager",
                pbjson::private::base64::encode(&self.manager).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPeer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "chain", "transceiver", "manager"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Chain,
            Transceiver,
            Manager,
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
                            "chain" => Ok(GeneratedField::Chain),
                            "transceiver" => Ok(GeneratedField::Transceiver),
                            "manager" => Ok(GeneratedField::Manager),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetPeer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetPeer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetPeer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut chain__ = None;
                let mut transceiver__ = None;
                let mut manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Chain => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            chain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Transceiver => {
                            if transceiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transceiver"));
                            }
                            transceiver__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSetPeer {
                    signer: signer__.unwrap_or_default(),
                    chain: chain__.unwrap_or_default(),
                    transceiver: transceiver__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetPeer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPeerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgSetPeerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPeerResponse {
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
            type Value = MsgSetPeerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgSetPeerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetPeerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetPeerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgSetPeerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgTransfer {
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
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.destination_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgTransfer", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.destination_chain_id != 0 {
            struct_ser.serialize_field("destinationChainId", &self.destination_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        if !self.recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipient",
                pbjson::private::base64::encode(&self.recipient).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "amount",
            "destination_chain_id",
            "destinationChainId",
            "destination_token",
            "destinationToken",
            "recipient",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Amount,
            DestinationChainId,
            DestinationToken,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "amount" => Ok(GeneratedField::Amount),
                            "destinationChainId" | "destination_chain_id" => {
                                Ok(GeneratedField::DestinationChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
                            }
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
            type Value = MsgTransfer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgTransfer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTransfer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut amount__ = None;
                let mut destination_chain_id__ = None;
                let mut destination_token__ = None;
                let mut recipient__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationChainId => {
                            if destination_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationChainId",
                                ));
                            }
                            destination_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgTransfer {
                    signer: signer__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    destination_chain_id: destination_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgTransfer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgTransferOwnership {
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
        if !self.new_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgTransferOwnership", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferOwnership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "new_owner", "newOwner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            NewOwner,
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
                            "newOwner" | "new_owner" => Ok(GeneratedField::NewOwner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferOwnership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgTransferOwnership")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgTransferOwnership, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut new_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewOwner => {
                            if new_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newOwner"));
                            }
                            new_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgTransferOwnership {
                    signer: signer__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgTransferOwnership",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgTransferOwnershipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("noble.dollar.portal.v1.MsgTransferOwnershipResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferOwnershipResponse {
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
            type Value = MsgTransferOwnershipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgTransferOwnershipResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgTransferOwnershipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransferOwnershipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgTransferOwnershipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgTransferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.MsgTransferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferResponse {
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
            type Value = MsgTransferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.MsgTransferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTransferResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransferResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.MsgTransferResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for OwnershipTransferred {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_owner.is_empty() {
            len += 1;
        }
        if !self.new_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.OwnershipTransferred", len)?;
        if !self.previous_owner.is_empty() {
            struct_ser.serialize_field("previousOwner", &self.previous_owner)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OwnershipTransferred {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["previous_owner", "previousOwner", "new_owner", "newOwner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousOwner,
            NewOwner,
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
                            "previousOwner" | "previous_owner" => Ok(GeneratedField::PreviousOwner),
                            "newOwner" | "new_owner" => Ok(GeneratedField::NewOwner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OwnershipTransferred;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.OwnershipTransferred")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OwnershipTransferred, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_owner__ = None;
                let mut new_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousOwner => {
                            if previous_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousOwner"));
                            }
                            previous_owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewOwner => {
                            if new_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newOwner"));
                            }
                            new_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OwnershipTransferred {
                    previous_owner: previous_owner__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.OwnershipTransferred",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Paused {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.Paused", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Paused {
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
            type Value = Paused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.Paused")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Paused, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Paused {})
            }
        }
        deserializer.deserialize_struct("noble.dollar.portal.v1.Paused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Peer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transceiver.is_empty() {
            len += 1;
        }
        if !self.manager.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.Peer", len)?;
        if !self.transceiver.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "transceiver",
                pbjson::private::base64::encode(&self.transceiver).as_str(),
            )?;
        }
        if !self.manager.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "manager",
                pbjson::private::base64::encode(&self.manager).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Peer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["transceiver", "manager"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transceiver,
            Manager,
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
                            "transceiver" => Ok(GeneratedField::Transceiver),
                            "manager" => Ok(GeneratedField::Manager),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Peer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.Peer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Peer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transceiver__ = None;
                let mut manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transceiver => {
                            if transceiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transceiver"));
                            }
                            transceiver__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Peer {
                    transceiver: transceiver__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.portal.v1.Peer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PeerUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain != 0 {
            len += 1;
        }
        if !self.old_transceiver.is_empty() {
            len += 1;
        }
        if !self.new_transceiver.is_empty() {
            len += 1;
        }
        if !self.old_manager.is_empty() {
            len += 1;
        }
        if !self.new_manager.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.PeerUpdated", len)?;
        if self.chain != 0 {
            struct_ser.serialize_field("chain", &self.chain)?;
        }
        if !self.old_transceiver.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "oldTransceiver",
                pbjson::private::base64::encode(&self.old_transceiver).as_str(),
            )?;
        }
        if !self.new_transceiver.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newTransceiver",
                pbjson::private::base64::encode(&self.new_transceiver).as_str(),
            )?;
        }
        if !self.old_manager.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "oldManager",
                pbjson::private::base64::encode(&self.old_manager).as_str(),
            )?;
        }
        if !self.new_manager.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newManager",
                pbjson::private::base64::encode(&self.new_manager).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PeerUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain",
            "old_transceiver",
            "oldTransceiver",
            "new_transceiver",
            "newTransceiver",
            "old_manager",
            "oldManager",
            "new_manager",
            "newManager",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chain,
            OldTransceiver,
            NewTransceiver,
            OldManager,
            NewManager,
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
                            "chain" => Ok(GeneratedField::Chain),
                            "oldTransceiver" | "old_transceiver" => {
                                Ok(GeneratedField::OldTransceiver)
                            }
                            "newTransceiver" | "new_transceiver" => {
                                Ok(GeneratedField::NewTransceiver)
                            }
                            "oldManager" | "old_manager" => Ok(GeneratedField::OldManager),
                            "newManager" | "new_manager" => Ok(GeneratedField::NewManager),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PeerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.PeerUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PeerUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain__ = None;
                let mut old_transceiver__ = None;
                let mut new_transceiver__ = None;
                let mut old_manager__ = None;
                let mut new_manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Chain => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            chain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OldTransceiver => {
                            if old_transceiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldTransceiver"));
                            }
                            old_transceiver__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewTransceiver => {
                            if new_transceiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newTransceiver"));
                            }
                            new_transceiver__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OldManager => {
                            if old_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldManager"));
                            }
                            old_manager__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewManager => {
                            if new_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newManager"));
                            }
                            new_manager__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PeerUpdated {
                    chain: chain__.unwrap_or_default(),
                    old_transceiver: old_transceiver__.unwrap_or_default(),
                    new_transceiver: new_transceiver__.unwrap_or_default(),
                    old_manager: old_manager__.unwrap_or_default(),
                    new_manager: new_manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.PeerUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryDestinationTokens {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.QueryDestinationTokens", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDestinationTokens {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDestinationTokens;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryDestinationTokens")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDestinationTokens, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryDestinationTokens {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryDestinationTokens",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryDestinationTokensResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.destination_tokens.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("noble.dollar.portal.v1.QueryDestinationTokensResponse", len)?;
        if !self.destination_tokens.is_empty() {
            struct_ser.serialize_field(
                "destinationTokens",
                &self
                    .destination_tokens
                    .iter()
                    .map(pbjson::private::base64::encode)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDestinationTokensResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["destination_tokens", "destinationTokens"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationTokens,
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
                            "destinationTokens" | "destination_tokens" => {
                                Ok(GeneratedField::DestinationTokens)
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
            type Value = QueryDestinationTokensResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryDestinationTokensResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDestinationTokensResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut destination_tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DestinationTokens => {
                            if destination_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationTokens"));
                            }
                            destination_tokens__ = Some(
                                map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(QueryDestinationTokensResponse {
                    destination_tokens: destination_tokens__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryDestinationTokensResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryNonce {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.QueryNonce", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryNonce {
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
            type Value = QueryNonce;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryNonce")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryNonce, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryNonce {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryNonce",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.QueryNonceResponse", len)?;
        if self.nonce != 0 {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryNonceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNonceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryNonceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryNonceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryNonceResponse {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryNonceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryOwner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.QueryOwner", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOwner {
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
            type Value = QueryOwner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryOwner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryOwner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryOwner {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryOwner",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryOwnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.QueryOwnerResponse", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOwnerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["owner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
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
                            "owner" => Ok(GeneratedField::Owner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOwnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryOwnerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryOwnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOwnerResponse {
                    owner: owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryOwnerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPaused {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.QueryPaused", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPaused {
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
            type Value = QueryPaused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryPaused")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPaused, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPaused {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryPaused",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPausedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.paused {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.QueryPausedResponse", len)?;
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPausedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["paused"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Paused,
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
                            "paused" => Ok(GeneratedField::Paused),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPausedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryPausedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPausedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut paused__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPausedResponse {
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPeers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.QueryPeers", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPeers {
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
            type Value = QueryPeers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryPeers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPeers, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPeers {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryPeers",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPeersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.peers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.QueryPeersResponse", len)?;
        if !self.peers.is_empty() {
            struct_ser.serialize_field("peers", &self.peers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPeersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["peers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Peers,
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
                            "peers" => Ok(GeneratedField::Peers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPeersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.QueryPeersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPeersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut peers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Peers => {
                            if peers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peers"));
                            }
                            peers__ = Some(
                                map_.next_value::<std::collections::HashMap<
                                    ::pbjson::private::NumberDeserialize<u32>,
                                    _,
                                >>()?
                                .into_iter()
                                .map(|(k, v)| (k.0, v))
                                .collect(),
                            );
                        }
                    }
                }
                Ok(QueryPeersResponse {
                    peers: peers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.QueryPeersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TransferRedeemed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.digest.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.TransferRedeemed", len)?;
        if !self.digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "digest",
                pbjson::private::base64::encode(&self.digest).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransferRedeemed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["digest"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Digest,
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
                            "digest" => Ok(GeneratedField::Digest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransferRedeemed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.TransferRedeemed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransferRedeemed, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut digest__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TransferRedeemed {
                    digest: digest__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.TransferRedeemed",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UsdnTokenSent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_token.is_empty() {
            len += 1;
        }
        if self.destination_chain_id != 0 {
            len += 1;
        }
        if !self.destination_token.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.message_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.v1.USDNTokenSent", len)?;
        if !self.source_token.is_empty() {
            struct_ser.serialize_field("sourceToken", &self.source_token)?;
        }
        if self.destination_chain_id != 0 {
            struct_ser.serialize_field("destinationChainId", &self.destination_chain_id)?;
        }
        if !self.destination_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationToken",
                pbjson::private::base64::encode(&self.destination_token).as_str(),
            )?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipient",
                pbjson::private::base64::encode(&self.recipient).as_str(),
            )?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.message_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageId",
                pbjson::private::base64::encode(&self.message_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsdnTokenSent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_token",
            "sourceToken",
            "destination_chain_id",
            "destinationChainId",
            "destination_token",
            "destinationToken",
            "sender",
            "recipient",
            "amount",
            "index",
            "message_id",
            "messageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceToken,
            DestinationChainId,
            DestinationToken,
            Sender,
            Recipient,
            Amount,
            Index,
            MessageId,
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
                            "sourceToken" | "source_token" => Ok(GeneratedField::SourceToken),
                            "destinationChainId" | "destination_chain_id" => {
                                Ok(GeneratedField::DestinationChainId)
                            }
                            "destinationToken" | "destination_token" => {
                                Ok(GeneratedField::DestinationToken)
                            }
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "index" => Ok(GeneratedField::Index),
                            "messageId" | "message_id" => Ok(GeneratedField::MessageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsdnTokenSent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.USDNTokenSent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsdnTokenSent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_token__ = None;
                let mut destination_chain_id__ = None;
                let mut destination_token__ = None;
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut amount__ = None;
                let mut index__ = None;
                let mut message_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceToken => {
                            if source_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceToken"));
                            }
                            source_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationChainId => {
                            if destination_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationChainId",
                                ));
                            }
                            destination_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationToken => {
                            if destination_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationToken"));
                            }
                            destination_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MessageId => {
                            if message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageId"));
                            }
                            message_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(UsdnTokenSent {
                    source_token: source_token__.unwrap_or_default(),
                    destination_chain_id: destination_chain_id__.unwrap_or_default(),
                    destination_token: destination_token__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    message_id: message_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.v1.USDNTokenSent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Unpaused {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.portal.v1.Unpaused", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Unpaused {
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
            type Value = Unpaused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.v1.Unpaused")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Unpaused, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Unpaused {})
            }
        }
        deserializer.deserialize_struct("noble.dollar.portal.v1.Unpaused", FIELDS, GeneratedVisitor)
    }
}
