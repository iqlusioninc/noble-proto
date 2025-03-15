// @generated
impl serde::Serialize for ManagerMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.ntt.v1.ManagerMessage", len)?;
        if !self.id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        if !self.sender.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "sender",
                pbjson::private::base64::encode(&self.sender).as_str(),
            )?;
        }
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "payload",
                pbjson::private::base64::encode(&self.payload).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManagerMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "sender", "payload"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Sender,
            Payload,
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
                            "id" => Ok(GeneratedField::Id),
                            "sender" => Ok(GeneratedField::Sender),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManagerMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.ntt.v1.ManagerMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManagerMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut sender__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
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
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ManagerMessage {
                    id: id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.ntt.v1.ManagerMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NativeTokenTransfer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount != 0 {
            len += 1;
        }
        if !self.source_token.is_empty() {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if self.to_chain != 0 {
            len += 1;
        }
        if !self.additional_payload.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.ntt.v1.NativeTokenTransfer", len)?;
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        if !self.source_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "sourceToken",
                pbjson::private::base64::encode(&self.source_token).as_str(),
            )?;
        }
        if !self.to.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("to", pbjson::private::base64::encode(&self.to).as_str())?;
        }
        if self.to_chain != 0 {
            struct_ser.serialize_field("toChain", &self.to_chain)?;
        }
        if !self.additional_payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "additionalPayload",
                pbjson::private::base64::encode(&self.additional_payload).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NativeTokenTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "source_token",
            "sourceToken",
            "to",
            "to_chain",
            "toChain",
            "additional_payload",
            "additionalPayload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            SourceToken,
            To,
            ToChain,
            AdditionalPayload,
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
                            "amount" => Ok(GeneratedField::Amount),
                            "sourceToken" | "source_token" => Ok(GeneratedField::SourceToken),
                            "to" => Ok(GeneratedField::To),
                            "toChain" | "to_chain" => Ok(GeneratedField::ToChain),
                            "additionalPayload" | "additional_payload" => {
                                Ok(GeneratedField::AdditionalPayload)
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
            type Value = NativeTokenTransfer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.ntt.v1.NativeTokenTransfer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NativeTokenTransfer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut source_token__ = None;
                let mut to__ = None;
                let mut to_chain__ = None;
                let mut additional_payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SourceToken => {
                            if source_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceToken"));
                            }
                            source_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ToChain => {
                            if to_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toChain"));
                            }
                            to_chain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AdditionalPayload => {
                            if additional_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalPayload"));
                            }
                            additional_payload__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(NativeTokenTransfer {
                    amount: amount__.unwrap_or_default(),
                    source_token: source_token__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    to_chain: to_chain__.unwrap_or_default(),
                    additional_payload: additional_payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.ntt.v1.NativeTokenTransfer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TransceiverMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_manager_address.is_empty() {
            len += 1;
        }
        if !self.recipient_manager_address.is_empty() {
            len += 1;
        }
        if !self.manager_payload.is_empty() {
            len += 1;
        }
        if !self.transceiver_payload.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.portal.ntt.v1.TransceiverMessage", len)?;
        if !self.source_manager_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "sourceManagerAddress",
                pbjson::private::base64::encode(&self.source_manager_address).as_str(),
            )?;
        }
        if !self.recipient_manager_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipientManagerAddress",
                pbjson::private::base64::encode(&self.recipient_manager_address).as_str(),
            )?;
        }
        if !self.manager_payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "managerPayload",
                pbjson::private::base64::encode(&self.manager_payload).as_str(),
            )?;
        }
        if !self.transceiver_payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "transceiverPayload",
                pbjson::private::base64::encode(&self.transceiver_payload).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransceiverMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_manager_address",
            "sourceManagerAddress",
            "recipient_manager_address",
            "recipientManagerAddress",
            "manager_payload",
            "managerPayload",
            "transceiver_payload",
            "transceiverPayload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceManagerAddress,
            RecipientManagerAddress,
            ManagerPayload,
            TransceiverPayload,
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
                            "sourceManagerAddress" | "source_manager_address" => {
                                Ok(GeneratedField::SourceManagerAddress)
                            }
                            "recipientManagerAddress" | "recipient_manager_address" => {
                                Ok(GeneratedField::RecipientManagerAddress)
                            }
                            "managerPayload" | "manager_payload" => {
                                Ok(GeneratedField::ManagerPayload)
                            }
                            "transceiverPayload" | "transceiver_payload" => {
                                Ok(GeneratedField::TransceiverPayload)
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
            type Value = TransceiverMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.portal.ntt.v1.TransceiverMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransceiverMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_manager_address__ = None;
                let mut recipient_manager_address__ = None;
                let mut manager_payload__ = None;
                let mut transceiver_payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceManagerAddress => {
                            if source_manager_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceManagerAddress",
                                ));
                            }
                            source_manager_address__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RecipientManagerAddress => {
                            if recipient_manager_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "recipientManagerAddress",
                                ));
                            }
                            recipient_manager_address__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ManagerPayload => {
                            if manager_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("managerPayload"));
                            }
                            manager_payload__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TransceiverPayload => {
                            if transceiver_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "transceiverPayload",
                                ));
                            }
                            transceiver_payload__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TransceiverMessage {
                    source_manager_address: source_manager_address__.unwrap_or_default(),
                    recipient_manager_address: recipient_manager_address__.unwrap_or_default(),
                    manager_payload: manager_payload__.unwrap_or_default(),
                    transceiver_payload: transceiver_payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.portal.ntt.v1.TransceiverMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
