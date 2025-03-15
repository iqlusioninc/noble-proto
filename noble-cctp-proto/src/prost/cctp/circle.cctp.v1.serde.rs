// @generated
impl serde::Serialize for Attester {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.Attester", len)?;
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Attester {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attester,
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
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Attester;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.Attester")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Attester, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Attester {
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.Attester", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttesterDisabled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.AttesterDisabled", len)?;
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttesterDisabled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attester,
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
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttesterDisabled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.AttesterDisabled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttesterDisabled, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AttesterDisabled {
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.AttesterDisabled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttesterEnabled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.AttesterEnabled", len)?;
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttesterEnabled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attester,
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
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttesterEnabled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.AttesterEnabled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttesterEnabled, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AttesterEnabled {
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.AttesterEnabled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttesterManagerUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_attester_manager.is_empty() {
            len += 1;
        }
        if !self.new_attester_manager.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.AttesterManagerUpdated", len)?;
        if !self.previous_attester_manager.is_empty() {
            struct_ser
                .serialize_field("previousAttesterManager", &self.previous_attester_manager)?;
        }
        if !self.new_attester_manager.is_empty() {
            struct_ser.serialize_field("newAttesterManager", &self.new_attester_manager)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttesterManagerUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_attester_manager",
            "previousAttesterManager",
            "new_attester_manager",
            "newAttesterManager",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousAttesterManager,
            NewAttesterManager,
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
                            "previousAttesterManager" | "previous_attester_manager" => {
                                Ok(GeneratedField::PreviousAttesterManager)
                            }
                            "newAttesterManager" | "new_attester_manager" => {
                                Ok(GeneratedField::NewAttesterManager)
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
            type Value = AttesterManagerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.AttesterManagerUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AttesterManagerUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_attester_manager__ = None;
                let mut new_attester_manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousAttesterManager => {
                            if previous_attester_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "previousAttesterManager",
                                ));
                            }
                            previous_attester_manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAttesterManager => {
                            if new_attester_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newAttesterManager",
                                ));
                            }
                            new_attester_manager__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AttesterManagerUpdated {
                    previous_attester_manager: previous_attester_manager__.unwrap_or_default(),
                    new_attester_manager: new_attester_manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.AttesterManagerUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BurnMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.burn_token.is_empty() {
            len += 1;
        }
        if !self.mint_recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.message_sender.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.BurnMessage", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.burn_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "burnToken",
                pbjson::private::base64::encode(&self.burn_token).as_str(),
            )?;
        }
        if !self.mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "mintRecipient",
                pbjson::private::base64::encode(&self.mint_recipient).as_str(),
            )?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.message_sender.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageSender",
                pbjson::private::base64::encode(&self.message_sender).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurnMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "burn_token",
            "burnToken",
            "mint_recipient",
            "mintRecipient",
            "amount",
            "message_sender",
            "messageSender",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            BurnToken,
            MintRecipient,
            Amount,
            MessageSender,
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
                            "version" => Ok(GeneratedField::Version),
                            "burnToken" | "burn_token" => Ok(GeneratedField::BurnToken),
                            "mintRecipient" | "mint_recipient" => Ok(GeneratedField::MintRecipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "messageSender" | "message_sender" => Ok(GeneratedField::MessageSender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BurnMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.BurnMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BurnMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut burn_token__ = None;
                let mut mint_recipient__ = None;
                let mut amount__ = None;
                let mut message_sender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BurnToken => {
                            if burn_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnToken"));
                            }
                            burn_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MintRecipient => {
                            if mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRecipient"));
                            }
                            mint_recipient__ = Some(
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
                        GeneratedField::MessageSender => {
                            if message_sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageSender"));
                            }
                            message_sender__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BurnMessage {
                    version: version__.unwrap_or_default(),
                    burn_token: burn_token__.unwrap_or_default(),
                    mint_recipient: mint_recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    message_sender: message_sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.BurnMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BurningAndMintingPaused {
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
            serializer.serialize_struct("circle.cctp.v1.BurningAndMintingPaused", len)?;
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurningAndMintingPaused {
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
            type Value = BurningAndMintingPaused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.BurningAndMintingPaused")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BurningAndMintingPaused, V::Error>
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
                Ok(BurningAndMintingPaused {
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.BurningAndMintingPaused",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BurningAndMintingPausedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.BurningAndMintingPausedEvent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurningAndMintingPausedEvent {
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
            type Value = BurningAndMintingPausedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.BurningAndMintingPausedEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BurningAndMintingPausedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BurningAndMintingPausedEvent {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.BurningAndMintingPausedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BurningAndMintingUnpausedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.BurningAndMintingUnpausedEvent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurningAndMintingUnpausedEvent {
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
            type Value = BurningAndMintingUnpausedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.BurningAndMintingUnpausedEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BurningAndMintingUnpausedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BurningAndMintingUnpausedEvent {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.BurningAndMintingUnpausedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DepositForBurn {
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
        if !self.burn_token.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.depositor.is_empty() {
            len += 1;
        }
        if !self.mint_recipient.is_empty() {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if !self.destination_token_messenger.is_empty() {
            len += 1;
        }
        if !self.destination_caller.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.DepositForBurn", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.burn_token.is_empty() {
            struct_ser.serialize_field("burnToken", &self.burn_token)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.depositor.is_empty() {
            struct_ser.serialize_field("depositor", &self.depositor)?;
        }
        if !self.mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "mintRecipient",
                pbjson::private::base64::encode(&self.mint_recipient).as_str(),
            )?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if !self.destination_token_messenger.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationTokenMessenger",
                pbjson::private::base64::encode(&self.destination_token_messenger).as_str(),
            )?;
        }
        if !self.destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationCaller",
                pbjson::private::base64::encode(&self.destination_caller).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DepositForBurn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nonce",
            "burn_token",
            "burnToken",
            "amount",
            "depositor",
            "mint_recipient",
            "mintRecipient",
            "destination_domain",
            "destinationDomain",
            "destination_token_messenger",
            "destinationTokenMessenger",
            "destination_caller",
            "destinationCaller",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            BurnToken,
            Amount,
            Depositor,
            MintRecipient,
            DestinationDomain,
            DestinationTokenMessenger,
            DestinationCaller,
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
                            "burnToken" | "burn_token" => Ok(GeneratedField::BurnToken),
                            "amount" => Ok(GeneratedField::Amount),
                            "depositor" => Ok(GeneratedField::Depositor),
                            "mintRecipient" | "mint_recipient" => Ok(GeneratedField::MintRecipient),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "destinationTokenMessenger" | "destination_token_messenger" => {
                                Ok(GeneratedField::DestinationTokenMessenger)
                            }
                            "destinationCaller" | "destination_caller" => {
                                Ok(GeneratedField::DestinationCaller)
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
            type Value = DepositForBurn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.DepositForBurn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DepositForBurn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut burn_token__ = None;
                let mut amount__ = None;
                let mut depositor__ = None;
                let mut mint_recipient__ = None;
                let mut destination_domain__ = None;
                let mut destination_token_messenger__ = None;
                let mut destination_caller__ = None;
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
                        GeneratedField::BurnToken => {
                            if burn_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnToken"));
                            }
                            burn_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Depositor => {
                            if depositor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositor"));
                            }
                            depositor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MintRecipient => {
                            if mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRecipient"));
                            }
                            mint_recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationTokenMessenger => {
                            if destination_token_messenger__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationTokenMessenger",
                                ));
                            }
                            destination_token_messenger__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationCaller => {
                            if destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCaller"));
                            }
                            destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(DepositForBurn {
                    nonce: nonce__.unwrap_or_default(),
                    burn_token: burn_token__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    depositor: depositor__.unwrap_or_default(),
                    mint_recipient: mint_recipient__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    destination_token_messenger: destination_token_messenger__.unwrap_or_default(),
                    destination_caller: destination_caller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.DepositForBurn", FIELDS, GeneratedVisitor)
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
        if !self.attester_manager.is_empty() {
            len += 1;
        }
        if !self.pauser.is_empty() {
            len += 1;
        }
        if !self.token_controller.is_empty() {
            len += 1;
        }
        if !self.attester_list.is_empty() {
            len += 1;
        }
        if !self.per_message_burn_limit_list.is_empty() {
            len += 1;
        }
        if self.burning_and_minting_paused.is_some() {
            len += 1;
        }
        if self.sending_and_receiving_messages_paused.is_some() {
            len += 1;
        }
        if self.max_message_body_size.is_some() {
            len += 1;
        }
        if self.next_available_nonce.is_some() {
            len += 1;
        }
        if self.signature_threshold.is_some() {
            len += 1;
        }
        if !self.token_pair_list.is_empty() {
            len += 1;
        }
        if !self.used_nonces_list.is_empty() {
            len += 1;
        }
        if !self.token_messenger_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.GenesisState", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.attester_manager.is_empty() {
            struct_ser.serialize_field("attesterManager", &self.attester_manager)?;
        }
        if !self.pauser.is_empty() {
            struct_ser.serialize_field("pauser", &self.pauser)?;
        }
        if !self.token_controller.is_empty() {
            struct_ser.serialize_field("tokenController", &self.token_controller)?;
        }
        if !self.attester_list.is_empty() {
            struct_ser.serialize_field("attesterList", &self.attester_list)?;
        }
        if !self.per_message_burn_limit_list.is_empty() {
            struct_ser
                .serialize_field("perMessageBurnLimitList", &self.per_message_burn_limit_list)?;
        }
        if let Some(v) = self.burning_and_minting_paused.as_ref() {
            struct_ser.serialize_field("burningAndMintingPaused", v)?;
        }
        if let Some(v) = self.sending_and_receiving_messages_paused.as_ref() {
            struct_ser.serialize_field("sendingAndReceivingMessagesPaused", v)?;
        }
        if let Some(v) = self.max_message_body_size.as_ref() {
            struct_ser.serialize_field("maxMessageBodySize", v)?;
        }
        if let Some(v) = self.next_available_nonce.as_ref() {
            struct_ser.serialize_field("nextAvailableNonce", v)?;
        }
        if let Some(v) = self.signature_threshold.as_ref() {
            struct_ser.serialize_field("signatureThreshold", v)?;
        }
        if !self.token_pair_list.is_empty() {
            struct_ser.serialize_field("tokenPairList", &self.token_pair_list)?;
        }
        if !self.used_nonces_list.is_empty() {
            struct_ser.serialize_field("usedNoncesList", &self.used_nonces_list)?;
        }
        if !self.token_messenger_list.is_empty() {
            struct_ser.serialize_field("tokenMessengerList", &self.token_messenger_list)?;
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
            "attester_manager",
            "attesterManager",
            "pauser",
            "token_controller",
            "tokenController",
            "attester_list",
            "attesterList",
            "per_message_burn_limit_list",
            "perMessageBurnLimitList",
            "burning_and_minting_paused",
            "burningAndMintingPaused",
            "sending_and_receiving_messages_paused",
            "sendingAndReceivingMessagesPaused",
            "max_message_body_size",
            "maxMessageBodySize",
            "next_available_nonce",
            "nextAvailableNonce",
            "signature_threshold",
            "signatureThreshold",
            "token_pair_list",
            "tokenPairList",
            "used_nonces_list",
            "usedNoncesList",
            "token_messenger_list",
            "tokenMessengerList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            AttesterManager,
            Pauser,
            TokenController,
            AttesterList,
            PerMessageBurnLimitList,
            BurningAndMintingPaused,
            SendingAndReceivingMessagesPaused,
            MaxMessageBodySize,
            NextAvailableNonce,
            SignatureThreshold,
            TokenPairList,
            UsedNoncesList,
            TokenMessengerList,
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
                            "attesterManager" | "attester_manager" => {
                                Ok(GeneratedField::AttesterManager)
                            }
                            "pauser" => Ok(GeneratedField::Pauser),
                            "tokenController" | "token_controller" => {
                                Ok(GeneratedField::TokenController)
                            }
                            "attesterList" | "attester_list" => Ok(GeneratedField::AttesterList),
                            "perMessageBurnLimitList" | "per_message_burn_limit_list" => {
                                Ok(GeneratedField::PerMessageBurnLimitList)
                            }
                            "burningAndMintingPaused" | "burning_and_minting_paused" => {
                                Ok(GeneratedField::BurningAndMintingPaused)
                            }
                            "sendingAndReceivingMessagesPaused"
                            | "sending_and_receiving_messages_paused" => {
                                Ok(GeneratedField::SendingAndReceivingMessagesPaused)
                            }
                            "maxMessageBodySize" | "max_message_body_size" => {
                                Ok(GeneratedField::MaxMessageBodySize)
                            }
                            "nextAvailableNonce" | "next_available_nonce" => {
                                Ok(GeneratedField::NextAvailableNonce)
                            }
                            "signatureThreshold" | "signature_threshold" => {
                                Ok(GeneratedField::SignatureThreshold)
                            }
                            "tokenPairList" | "token_pair_list" => {
                                Ok(GeneratedField::TokenPairList)
                            }
                            "usedNoncesList" | "used_nonces_list" => {
                                Ok(GeneratedField::UsedNoncesList)
                            }
                            "tokenMessengerList" | "token_messenger_list" => {
                                Ok(GeneratedField::TokenMessengerList)
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
                formatter.write_str("struct circle.cctp.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut attester_manager__ = None;
                let mut pauser__ = None;
                let mut token_controller__ = None;
                let mut attester_list__ = None;
                let mut per_message_burn_limit_list__ = None;
                let mut burning_and_minting_paused__ = None;
                let mut sending_and_receiving_messages_paused__ = None;
                let mut max_message_body_size__ = None;
                let mut next_available_nonce__ = None;
                let mut signature_threshold__ = None;
                let mut token_pair_list__ = None;
                let mut used_nonces_list__ = None;
                let mut token_messenger_list__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttesterManager => {
                            if attester_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attesterManager"));
                            }
                            attester_manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pauser => {
                            if pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauser"));
                            }
                            pauser__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenController => {
                            if token_controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenController"));
                            }
                            token_controller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttesterList => {
                            if attester_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attesterList"));
                            }
                            attester_list__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PerMessageBurnLimitList => {
                            if per_message_burn_limit_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "perMessageBurnLimitList",
                                ));
                            }
                            per_message_burn_limit_list__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BurningAndMintingPaused => {
                            if burning_and_minting_paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "burningAndMintingPaused",
                                ));
                            }
                            burning_and_minting_paused__ = map_.next_value()?;
                        }
                        GeneratedField::SendingAndReceivingMessagesPaused => {
                            if sending_and_receiving_messages_paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sendingAndReceivingMessagesPaused",
                                ));
                            }
                            sending_and_receiving_messages_paused__ = map_.next_value()?;
                        }
                        GeneratedField::MaxMessageBodySize => {
                            if max_message_body_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxMessageBodySize",
                                ));
                            }
                            max_message_body_size__ = map_.next_value()?;
                        }
                        GeneratedField::NextAvailableNonce => {
                            if next_available_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "nextAvailableNonce",
                                ));
                            }
                            next_available_nonce__ = map_.next_value()?;
                        }
                        GeneratedField::SignatureThreshold => {
                            if signature_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "signatureThreshold",
                                ));
                            }
                            signature_threshold__ = map_.next_value()?;
                        }
                        GeneratedField::TokenPairList => {
                            if token_pair_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenPairList"));
                            }
                            token_pair_list__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsedNoncesList => {
                            if used_nonces_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedNoncesList"));
                            }
                            used_nonces_list__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenMessengerList => {
                            if token_messenger_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "tokenMessengerList",
                                ));
                            }
                            token_messenger_list__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    owner: owner__.unwrap_or_default(),
                    attester_manager: attester_manager__.unwrap_or_default(),
                    pauser: pauser__.unwrap_or_default(),
                    token_controller: token_controller__.unwrap_or_default(),
                    attester_list: attester_list__.unwrap_or_default(),
                    per_message_burn_limit_list: per_message_burn_limit_list__.unwrap_or_default(),
                    burning_and_minting_paused: burning_and_minting_paused__,
                    sending_and_receiving_messages_paused: sending_and_receiving_messages_paused__,
                    max_message_body_size: max_message_body_size__,
                    next_available_nonce: next_available_nonce__,
                    signature_threshold: signature_threshold__,
                    token_pair_list: token_pair_list__.unwrap_or_default(),
                    used_nonces_list: used_nonces_list__.unwrap_or_default(),
                    token_messenger_list: token_messenger_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MaxMessageBodySize {
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
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MaxMessageBodySize", len)?;
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MaxMessageBodySize {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MaxMessageBodySize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MaxMessageBodySize")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MaxMessageBodySize, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
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
                    }
                }
                Ok(MaxMessageBodySize {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MaxMessageBodySize",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MaxMessageBodySizeUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.new_max_message_body_size != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MaxMessageBodySizeUpdated", len)?;
        if self.new_max_message_body_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newMaxMessageBodySize",
                ToString::to_string(&self.new_max_message_body_size).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MaxMessageBodySizeUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["new_max_message_body_size", "newMaxMessageBodySize"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewMaxMessageBodySize,
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
                            "newMaxMessageBodySize" | "new_max_message_body_size" => {
                                Ok(GeneratedField::NewMaxMessageBodySize)
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
            type Value = MaxMessageBodySizeUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MaxMessageBodySizeUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MaxMessageBodySizeUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut new_max_message_body_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NewMaxMessageBodySize => {
                            if new_max_message_body_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newMaxMessageBodySize",
                                ));
                            }
                            new_max_message_body_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MaxMessageBodySizeUpdated {
                    new_max_message_body_size: new_max_message_body_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MaxMessageBodySizeUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Message {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.source_domain != 0 {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.destination_caller.is_empty() {
            len += 1;
        }
        if !self.message_body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.Message", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.source_domain != 0 {
            struct_ser.serialize_field("sourceDomain", &self.source_domain)?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
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
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipient",
                pbjson::private::base64::encode(&self.recipient).as_str(),
            )?;
        }
        if !self.destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationCaller",
                pbjson::private::base64::encode(&self.destination_caller).as_str(),
            )?;
        }
        if !self.message_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageBody",
                pbjson::private::base64::encode(&self.message_body).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Message {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "source_domain",
            "sourceDomain",
            "destination_domain",
            "destinationDomain",
            "nonce",
            "sender",
            "recipient",
            "destination_caller",
            "destinationCaller",
            "message_body",
            "messageBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            SourceDomain,
            DestinationDomain,
            Nonce,
            Sender,
            Recipient,
            DestinationCaller,
            MessageBody,
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
                            "version" => Ok(GeneratedField::Version),
                            "sourceDomain" | "source_domain" => Ok(GeneratedField::SourceDomain),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "nonce" => Ok(GeneratedField::Nonce),
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "destinationCaller" | "destination_caller" => {
                                Ok(GeneratedField::DestinationCaller)
                            }
                            "messageBody" | "message_body" => Ok(GeneratedField::MessageBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.Message")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Message, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut source_domain__ = None;
                let mut destination_domain__ = None;
                let mut nonce__ = None;
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut destination_caller__ = None;
                let mut message_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SourceDomain => {
                            if source_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDomain"));
                            }
                            source_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
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
                            recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationCaller => {
                            if destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCaller"));
                            }
                            destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MessageBody => {
                            if message_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageBody"));
                            }
                            message_body__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Message {
                    version: version__.unwrap_or_default(),
                    source_domain: source_domain__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    destination_caller: destination_caller__.unwrap_or_default(),
                    message_body: message_body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.Message", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageReceived {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.caller.is_empty() {
            len += 1;
        }
        if self.source_domain != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.message_body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MessageReceived", len)?;
        if !self.caller.is_empty() {
            struct_ser.serialize_field("caller", &self.caller)?;
        }
        if self.source_domain != 0 {
            struct_ser.serialize_field("sourceDomain", &self.source_domain)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.sender.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "sender",
                pbjson::private::base64::encode(&self.sender).as_str(),
            )?;
        }
        if !self.message_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageBody",
                pbjson::private::base64::encode(&self.message_body).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageReceived {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "caller",
            "source_domain",
            "sourceDomain",
            "nonce",
            "sender",
            "message_body",
            "messageBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Caller,
            SourceDomain,
            Nonce,
            Sender,
            MessageBody,
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
                            "caller" => Ok(GeneratedField::Caller),
                            "sourceDomain" | "source_domain" => Ok(GeneratedField::SourceDomain),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "sender" => Ok(GeneratedField::Sender),
                            "messageBody" | "message_body" => Ok(GeneratedField::MessageBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageReceived;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MessageReceived")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageReceived, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut caller__ = None;
                let mut source_domain__ = None;
                let mut nonce__ = None;
                let mut sender__ = None;
                let mut message_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Caller => {
                            if caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caller"));
                            }
                            caller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceDomain => {
                            if source_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDomain"));
                            }
                            source_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
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
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MessageBody => {
                            if message_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageBody"));
                            }
                            message_body__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MessageReceived {
                    caller: caller__.unwrap_or_default(),
                    source_domain: source_domain__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    message_body: message_body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MessageReceived", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageSent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MessageSent", len)?;
        if !self.message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "message",
                pbjson::private::base64::encode(&self.message).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageSent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["message"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageSent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MessageSent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageSent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MessageSent {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MessageSent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MintAndWithdraw {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mint_recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.mint_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MintAndWithdraw", len)?;
        if !self.mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "mintRecipient",
                pbjson::private::base64::encode(&self.mint_recipient).as_str(),
            )?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.mint_token.is_empty() {
            struct_ser.serialize_field("mintToken", &self.mint_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MintAndWithdraw {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mint_recipient",
            "mintRecipient",
            "amount",
            "mint_token",
            "mintToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MintRecipient,
            Amount,
            MintToken,
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
                            "mintRecipient" | "mint_recipient" => Ok(GeneratedField::MintRecipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "mintToken" | "mint_token" => Ok(GeneratedField::MintToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MintAndWithdraw;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MintAndWithdraw")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MintAndWithdraw, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut mint_recipient__ = None;
                let mut amount__ = None;
                let mut mint_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MintRecipient => {
                            if mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRecipient"));
                            }
                            mint_recipient__ = Some(
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
                        GeneratedField::MintToken => {
                            if mint_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintToken"));
                            }
                            mint_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MintAndWithdraw {
                    mint_recipient: mint_recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    mint_token: mint_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MintAndWithdraw", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAcceptOwner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MsgAcceptOwner", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptOwner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAcceptOwner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgAcceptOwner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAcceptOwner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAcceptOwner {
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MsgAcceptOwner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAcceptOwnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgAcceptOwnerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptOwnerResponse {
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
            type Value = MsgAcceptOwnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgAcceptOwnerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAcceptOwnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAcceptOwnerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgAcceptOwnerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddRemoteTokenMessenger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.domain_id != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgAddRemoteTokenMessenger", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.domain_id != 0 {
            struct_ser.serialize_field("domainId", &self.domain_id)?;
        }
        if !self.address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "address",
                pbjson::private::base64::encode(&self.address).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRemoteTokenMessenger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "domain_id", "domainId", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            DomainId,
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
                            "from" => Ok(GeneratedField::From),
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
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
            type Value = MsgAddRemoteTokenMessenger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgAddRemoteTokenMessenger")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddRemoteTokenMessenger, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut domain_id__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgAddRemoteTokenMessenger {
                    from: from__.unwrap_or_default(),
                    domain_id: domain_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgAddRemoteTokenMessenger",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddRemoteTokenMessengerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgAddRemoteTokenMessengerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRemoteTokenMessengerResponse {
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
            type Value = MsgAddRemoteTokenMessengerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgAddRemoteTokenMessengerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddRemoteTokenMessengerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddRemoteTokenMessengerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgAddRemoteTokenMessengerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDepositForBurn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if !self.mint_recipient.is_empty() {
            len += 1;
        }
        if !self.burn_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgDepositForBurn", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if !self.mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "mintRecipient",
                pbjson::private::base64::encode(&self.mint_recipient).as_str(),
            )?;
        }
        if !self.burn_token.is_empty() {
            struct_ser.serialize_field("burnToken", &self.burn_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDepositForBurn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "amount",
            "destination_domain",
            "destinationDomain",
            "mint_recipient",
            "mintRecipient",
            "burn_token",
            "burnToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Amount,
            DestinationDomain,
            MintRecipient,
            BurnToken,
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
                            "from" => Ok(GeneratedField::From),
                            "amount" => Ok(GeneratedField::Amount),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "mintRecipient" | "mint_recipient" => Ok(GeneratedField::MintRecipient),
                            "burnToken" | "burn_token" => Ok(GeneratedField::BurnToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDepositForBurn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDepositForBurn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDepositForBurn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut amount__ = None;
                let mut destination_domain__ = None;
                let mut mint_recipient__ = None;
                let mut burn_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MintRecipient => {
                            if mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRecipient"));
                            }
                            mint_recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BurnToken => {
                            if burn_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnToken"));
                            }
                            burn_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgDepositForBurn {
                    from: from__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    mint_recipient: mint_recipient__.unwrap_or_default(),
                    burn_token: burn_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDepositForBurn",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDepositForBurnResponse {
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
            serializer.serialize_struct("circle.cctp.v1.MsgDepositForBurnResponse", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDepositForBurnResponse {
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
            type Value = MsgDepositForBurnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDepositForBurnResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDepositForBurnResponse, V::Error>
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
                Ok(MsgDepositForBurnResponse {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDepositForBurnResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDepositForBurnWithCaller {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if !self.mint_recipient.is_empty() {
            len += 1;
        }
        if !self.burn_token.is_empty() {
            len += 1;
        }
        if !self.destination_caller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgDepositForBurnWithCaller", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if !self.mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "mintRecipient",
                pbjson::private::base64::encode(&self.mint_recipient).as_str(),
            )?;
        }
        if !self.burn_token.is_empty() {
            struct_ser.serialize_field("burnToken", &self.burn_token)?;
        }
        if !self.destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationCaller",
                pbjson::private::base64::encode(&self.destination_caller).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDepositForBurnWithCaller {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "amount",
            "destination_domain",
            "destinationDomain",
            "mint_recipient",
            "mintRecipient",
            "burn_token",
            "burnToken",
            "destination_caller",
            "destinationCaller",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Amount,
            DestinationDomain,
            MintRecipient,
            BurnToken,
            DestinationCaller,
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
                            "from" => Ok(GeneratedField::From),
                            "amount" => Ok(GeneratedField::Amount),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "mintRecipient" | "mint_recipient" => Ok(GeneratedField::MintRecipient),
                            "burnToken" | "burn_token" => Ok(GeneratedField::BurnToken),
                            "destinationCaller" | "destination_caller" => {
                                Ok(GeneratedField::DestinationCaller)
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
            type Value = MsgDepositForBurnWithCaller;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDepositForBurnWithCaller")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDepositForBurnWithCaller, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut amount__ = None;
                let mut destination_domain__ = None;
                let mut mint_recipient__ = None;
                let mut burn_token__ = None;
                let mut destination_caller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MintRecipient => {
                            if mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRecipient"));
                            }
                            mint_recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BurnToken => {
                            if burn_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnToken"));
                            }
                            burn_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationCaller => {
                            if destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCaller"));
                            }
                            destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgDepositForBurnWithCaller {
                    from: from__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    mint_recipient: mint_recipient__.unwrap_or_default(),
                    burn_token: burn_token__.unwrap_or_default(),
                    destination_caller: destination_caller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDepositForBurnWithCaller",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDepositForBurnWithCallerResponse {
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
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgDepositForBurnWithCallerResponse", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDepositForBurnWithCallerResponse {
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
            type Value = MsgDepositForBurnWithCallerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDepositForBurnWithCallerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDepositForBurnWithCallerResponse, V::Error>
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
                Ok(MsgDepositForBurnWithCallerResponse {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDepositForBurnWithCallerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDisableAttester {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgDisableAttester", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDisableAttester {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Attester,
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
                            "from" => Ok(GeneratedField::From),
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDisableAttester;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDisableAttester")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDisableAttester, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgDisableAttester {
                    from: from__.unwrap_or_default(),
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDisableAttester",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDisableAttesterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgDisableAttesterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDisableAttesterResponse {
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
            type Value = MsgDisableAttesterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgDisableAttesterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDisableAttesterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDisableAttesterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgDisableAttesterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgEnableAttester {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgEnableAttester", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEnableAttester {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Attester,
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
                            "from" => Ok(GeneratedField::From),
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgEnableAttester;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgEnableAttester")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgEnableAttester, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgEnableAttester {
                    from: from__.unwrap_or_default(),
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgEnableAttester",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgEnableAttesterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgEnableAttesterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEnableAttesterResponse {
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
            type Value = MsgEnableAttesterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgEnableAttesterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgEnableAttesterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgEnableAttesterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgEnableAttesterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLinkTokenPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        if !self.local_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MsgLinkTokenPair", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteToken",
                pbjson::private::base64::encode(&self.remote_token).as_str(),
            )?;
        }
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkTokenPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
            "local_token",
            "localToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            RemoteDomain,
            RemoteToken,
            LocalToken,
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
                            "from" => Ok(GeneratedField::From),
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLinkTokenPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgLinkTokenPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgLinkTokenPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                let mut local_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgLinkTokenPair {
                    from: from__.unwrap_or_default(),
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                    local_token: local_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MsgLinkTokenPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgLinkTokenPairResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgLinkTokenPairResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkTokenPairResponse {
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
            type Value = MsgLinkTokenPairResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgLinkTokenPairResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgLinkTokenPairResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLinkTokenPairResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgLinkTokenPairResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPauseBurningAndMinting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgPauseBurningAndMinting", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPauseBurningAndMinting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgPauseBurningAndMinting;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgPauseBurningAndMinting")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgPauseBurningAndMinting, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgPauseBurningAndMinting {
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgPauseBurningAndMinting",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPauseBurningAndMintingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgPauseBurningAndMintingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPauseBurningAndMintingResponse {
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
            type Value = MsgPauseBurningAndMintingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgPauseBurningAndMintingResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgPauseBurningAndMintingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgPauseBurningAndMintingResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgPauseBurningAndMintingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPauseSendingAndReceivingMessages {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgPauseSendingAndReceivingMessages", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPauseSendingAndReceivingMessages {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgPauseSendingAndReceivingMessages;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgPauseSendingAndReceivingMessages")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgPauseSendingAndReceivingMessages, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgPauseSendingAndReceivingMessages {
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgPauseSendingAndReceivingMessages",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPauseSendingAndReceivingMessagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "circle.cctp.v1.MsgPauseSendingAndReceivingMessagesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPauseSendingAndReceivingMessagesResponse {
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
            type Value = MsgPauseSendingAndReceivingMessagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct circle.cctp.v1.MsgPauseSendingAndReceivingMessagesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgPauseSendingAndReceivingMessagesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgPauseSendingAndReceivingMessagesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgPauseSendingAndReceivingMessagesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReceiveMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.attestation.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReceiveMessage", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "message",
                pbjson::private::base64::encode(&self.message).as_str(),
            )?;
        }
        if !self.attestation.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "attestation",
                pbjson::private::base64::encode(&self.attestation).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReceiveMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "message", "attestation"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Message,
            Attestation,
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
                            "from" => Ok(GeneratedField::From),
                            "message" => Ok(GeneratedField::Message),
                            "attestation" => Ok(GeneratedField::Attestation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgReceiveMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReceiveMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgReceiveMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut message__ = None;
                let mut attestation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Attestation => {
                            if attestation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestation"));
                            }
                            attestation__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgReceiveMessage {
                    from: from__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    attestation: attestation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReceiveMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReceiveMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReceiveMessageResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReceiveMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["success"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgReceiveMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReceiveMessageResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgReceiveMessageResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgReceiveMessageResponse {
                    success: success__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReceiveMessageResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveRemoteTokenMessenger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.domain_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgRemoveRemoteTokenMessenger", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.domain_id != 0 {
            struct_ser.serialize_field("domainId", &self.domain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRemoteTokenMessenger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "domain_id", "domainId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            DomainId,
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
                            "from" => Ok(GeneratedField::From),
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveRemoteTokenMessenger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgRemoveRemoteTokenMessenger")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveRemoteTokenMessenger, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut domain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgRemoveRemoteTokenMessenger {
                    from: from__.unwrap_or_default(),
                    domain_id: domain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgRemoveRemoteTokenMessenger",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveRemoteTokenMessengerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgRemoveRemoteTokenMessengerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRemoteTokenMessengerResponse {
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
            type Value = MsgRemoveRemoteTokenMessengerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgRemoveRemoteTokenMessengerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveRemoteTokenMessengerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveRemoteTokenMessengerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgRemoveRemoteTokenMessengerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReplaceDepositForBurn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.original_message.is_empty() {
            len += 1;
        }
        if !self.original_attestation.is_empty() {
            len += 1;
        }
        if !self.new_destination_caller.is_empty() {
            len += 1;
        }
        if !self.new_mint_recipient.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReplaceDepositForBurn", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.original_message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "originalMessage",
                pbjson::private::base64::encode(&self.original_message).as_str(),
            )?;
        }
        if !self.original_attestation.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "originalAttestation",
                pbjson::private::base64::encode(&self.original_attestation).as_str(),
            )?;
        }
        if !self.new_destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newDestinationCaller",
                pbjson::private::base64::encode(&self.new_destination_caller).as_str(),
            )?;
        }
        if !self.new_mint_recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newMintRecipient",
                pbjson::private::base64::encode(&self.new_mint_recipient).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReplaceDepositForBurn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "original_message",
            "originalMessage",
            "original_attestation",
            "originalAttestation",
            "new_destination_caller",
            "newDestinationCaller",
            "new_mint_recipient",
            "newMintRecipient",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            OriginalMessage,
            OriginalAttestation,
            NewDestinationCaller,
            NewMintRecipient,
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
                            "from" => Ok(GeneratedField::From),
                            "originalMessage" | "original_message" => {
                                Ok(GeneratedField::OriginalMessage)
                            }
                            "originalAttestation" | "original_attestation" => {
                                Ok(GeneratedField::OriginalAttestation)
                            }
                            "newDestinationCaller" | "new_destination_caller" => {
                                Ok(GeneratedField::NewDestinationCaller)
                            }
                            "newMintRecipient" | "new_mint_recipient" => {
                                Ok(GeneratedField::NewMintRecipient)
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
            type Value = MsgReplaceDepositForBurn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReplaceDepositForBurn")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgReplaceDepositForBurn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut original_message__ = None;
                let mut original_attestation__ = None;
                let mut new_destination_caller__ = None;
                let mut new_mint_recipient__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginalMessage => {
                            if original_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalMessage"));
                            }
                            original_message__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OriginalAttestation => {
                            if original_attestation__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originalAttestation",
                                ));
                            }
                            original_attestation__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewDestinationCaller => {
                            if new_destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newDestinationCaller",
                                ));
                            }
                            new_destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewMintRecipient => {
                            if new_mint_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newMintRecipient"));
                            }
                            new_mint_recipient__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgReplaceDepositForBurn {
                    from: from__.unwrap_or_default(),
                    original_message: original_message__.unwrap_or_default(),
                    original_attestation: original_attestation__.unwrap_or_default(),
                    new_destination_caller: new_destination_caller__.unwrap_or_default(),
                    new_mint_recipient: new_mint_recipient__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReplaceDepositForBurn",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReplaceDepositForBurnResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReplaceDepositForBurnResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReplaceDepositForBurnResponse {
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
            type Value = MsgReplaceDepositForBurnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReplaceDepositForBurnResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgReplaceDepositForBurnResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgReplaceDepositForBurnResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReplaceDepositForBurnResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReplaceMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.original_message.is_empty() {
            len += 1;
        }
        if !self.original_attestation.is_empty() {
            len += 1;
        }
        if !self.new_message_body.is_empty() {
            len += 1;
        }
        if !self.new_destination_caller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReplaceMessage", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.original_message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "originalMessage",
                pbjson::private::base64::encode(&self.original_message).as_str(),
            )?;
        }
        if !self.original_attestation.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "originalAttestation",
                pbjson::private::base64::encode(&self.original_attestation).as_str(),
            )?;
        }
        if !self.new_message_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newMessageBody",
                pbjson::private::base64::encode(&self.new_message_body).as_str(),
            )?;
        }
        if !self.new_destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newDestinationCaller",
                pbjson::private::base64::encode(&self.new_destination_caller).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReplaceMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "original_message",
            "originalMessage",
            "original_attestation",
            "originalAttestation",
            "new_message_body",
            "newMessageBody",
            "new_destination_caller",
            "newDestinationCaller",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            OriginalMessage,
            OriginalAttestation,
            NewMessageBody,
            NewDestinationCaller,
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
                            "from" => Ok(GeneratedField::From),
                            "originalMessage" | "original_message" => {
                                Ok(GeneratedField::OriginalMessage)
                            }
                            "originalAttestation" | "original_attestation" => {
                                Ok(GeneratedField::OriginalAttestation)
                            }
                            "newMessageBody" | "new_message_body" => {
                                Ok(GeneratedField::NewMessageBody)
                            }
                            "newDestinationCaller" | "new_destination_caller" => {
                                Ok(GeneratedField::NewDestinationCaller)
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
            type Value = MsgReplaceMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReplaceMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgReplaceMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut original_message__ = None;
                let mut original_attestation__ = None;
                let mut new_message_body__ = None;
                let mut new_destination_caller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginalMessage => {
                            if original_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalMessage"));
                            }
                            original_message__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OriginalAttestation => {
                            if original_attestation__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originalAttestation",
                                ));
                            }
                            original_attestation__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewMessageBody => {
                            if new_message_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newMessageBody"));
                            }
                            new_message_body__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewDestinationCaller => {
                            if new_destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newDestinationCaller",
                                ));
                            }
                            new_destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgReplaceMessage {
                    from: from__.unwrap_or_default(),
                    original_message: original_message__.unwrap_or_default(),
                    original_attestation: original_attestation__.unwrap_or_default(),
                    new_message_body: new_message_body__.unwrap_or_default(),
                    new_destination_caller: new_destination_caller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReplaceMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReplaceMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgReplaceMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReplaceMessageResponse {
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
            type Value = MsgReplaceMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgReplaceMessageResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgReplaceMessageResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgReplaceMessageResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgReplaceMessageResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSendMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.message_body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MsgSendMessage", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if !self.recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipient",
                pbjson::private::base64::encode(&self.recipient).as_str(),
            )?;
        }
        if !self.message_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageBody",
                pbjson::private::base64::encode(&self.message_body).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSendMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "destination_domain",
            "destinationDomain",
            "recipient",
            "message_body",
            "messageBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            DestinationDomain,
            Recipient,
            MessageBody,
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
                            "from" => Ok(GeneratedField::From),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "recipient" => Ok(GeneratedField::Recipient),
                            "messageBody" | "message_body" => Ok(GeneratedField::MessageBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSendMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSendMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSendMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut destination_domain__ = None;
                let mut recipient__ = None;
                let mut message_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
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
                        GeneratedField::MessageBody => {
                            if message_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageBody"));
                            }
                            message_body__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSendMessage {
                    from: from__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    message_body: message_body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MsgSendMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSendMessageResponse {
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
            serializer.serialize_struct("circle.cctp.v1.MsgSendMessageResponse", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSendMessageResponse {
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
            type Value = MsgSendMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSendMessageResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSendMessageResponse, V::Error>
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
                Ok(MsgSendMessageResponse {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgSendMessageResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSendMessageWithCaller {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.destination_domain != 0 {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.message_body.is_empty() {
            len += 1;
        }
        if !self.destination_caller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgSendMessageWithCaller", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.destination_domain != 0 {
            struct_ser.serialize_field("destinationDomain", &self.destination_domain)?;
        }
        if !self.recipient.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "recipient",
                pbjson::private::base64::encode(&self.recipient).as_str(),
            )?;
        }
        if !self.message_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageBody",
                pbjson::private::base64::encode(&self.message_body).as_str(),
            )?;
        }
        if !self.destination_caller.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "destinationCaller",
                pbjson::private::base64::encode(&self.destination_caller).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSendMessageWithCaller {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "destination_domain",
            "destinationDomain",
            "recipient",
            "message_body",
            "messageBody",
            "destination_caller",
            "destinationCaller",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            DestinationDomain,
            Recipient,
            MessageBody,
            DestinationCaller,
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
                            "from" => Ok(GeneratedField::From),
                            "destinationDomain" | "destination_domain" => {
                                Ok(GeneratedField::DestinationDomain)
                            }
                            "recipient" => Ok(GeneratedField::Recipient),
                            "messageBody" | "message_body" => Ok(GeneratedField::MessageBody),
                            "destinationCaller" | "destination_caller" => {
                                Ok(GeneratedField::DestinationCaller)
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
            type Value = MsgSendMessageWithCaller;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSendMessageWithCaller")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSendMessageWithCaller, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut destination_domain__ = None;
                let mut recipient__ = None;
                let mut message_body__ = None;
                let mut destination_caller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestinationDomain => {
                            if destination_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationDomain"));
                            }
                            destination_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
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
                        GeneratedField::MessageBody => {
                            if message_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageBody"));
                            }
                            message_body__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DestinationCaller => {
                            if destination_caller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCaller"));
                            }
                            destination_caller__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSendMessageWithCaller {
                    from: from__.unwrap_or_default(),
                    destination_domain: destination_domain__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    message_body: message_body__.unwrap_or_default(),
                    destination_caller: destination_caller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgSendMessageWithCaller",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSendMessageWithCallerResponse {
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
            serializer.serialize_struct("circle.cctp.v1.MsgSendMessageWithCallerResponse", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSendMessageWithCallerResponse {
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
            type Value = MsgSendMessageWithCallerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSendMessageWithCallerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSendMessageWithCallerResponse, V::Error>
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
                Ok(MsgSendMessageWithCallerResponse {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgSendMessageWithCallerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetMaxBurnAmountPerMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.local_token.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgSetMaxBurnAmountPerMessage", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetMaxBurnAmountPerMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "local_token", "localToken", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            LocalToken,
            Amount,
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
                            "from" => Ok(GeneratedField::From),
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetMaxBurnAmountPerMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSetMaxBurnAmountPerMessage")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetMaxBurnAmountPerMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut local_token__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetMaxBurnAmountPerMessage {
                    from: from__.unwrap_or_default(),
                    local_token: local_token__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgSetMaxBurnAmountPerMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetMaxBurnAmountPerMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgSetMaxBurnAmountPerMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetMaxBurnAmountPerMessageResponse {
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
            type Value = MsgSetMaxBurnAmountPerMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgSetMaxBurnAmountPerMessageResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetMaxBurnAmountPerMessageResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetMaxBurnAmountPerMessageResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgSetMaxBurnAmountPerMessageResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkTokenPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        if !self.local_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUnlinkTokenPair", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteToken",
                pbjson::private::base64::encode(&self.remote_token).as_str(),
            )?;
        }
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkTokenPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
            "local_token",
            "localToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            RemoteDomain,
            RemoteToken,
            LocalToken,
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
                            "from" => Ok(GeneratedField::From),
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnlinkTokenPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUnlinkTokenPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnlinkTokenPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                let mut local_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUnlinkTokenPair {
                    from: from__.unwrap_or_default(),
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                    local_token: local_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnlinkTokenPair",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkTokenPairResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUnlinkTokenPairResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkTokenPairResponse {
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
            type Value = MsgUnlinkTokenPairResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUnlinkTokenPairResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnlinkTokenPairResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnlinkTokenPairResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnlinkTokenPairResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpauseBurningAndMinting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUnpauseBurningAndMinting", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpauseBurningAndMinting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnpauseBurningAndMinting;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUnpauseBurningAndMinting")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnpauseBurningAndMinting, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUnpauseBurningAndMinting {
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnpauseBurningAndMinting",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpauseBurningAndMintingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgUnpauseBurningAndMintingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpauseBurningAndMintingResponse {
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
            type Value = MsgUnpauseBurningAndMintingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUnpauseBurningAndMintingResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnpauseBurningAndMintingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnpauseBurningAndMintingResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnpauseBurningAndMintingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpauseSendingAndReceivingMessages {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgUnpauseSendingAndReceivingMessages", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpauseSendingAndReceivingMessages {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnpauseSendingAndReceivingMessages;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUnpauseSendingAndReceivingMessages")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnpauseSendingAndReceivingMessages, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUnpauseSendingAndReceivingMessages {
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnpauseSendingAndReceivingMessages",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpauseSendingAndReceivingMessagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "circle.cctp.v1.MsgUnpauseSendingAndReceivingMessagesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpauseSendingAndReceivingMessagesResponse {
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
            type Value = MsgUnpauseSendingAndReceivingMessagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct circle.cctp.v1.MsgUnpauseSendingAndReceivingMessagesResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnpauseSendingAndReceivingMessagesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnpauseSendingAndReceivingMessagesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUnpauseSendingAndReceivingMessagesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateAttesterManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.new_attester_manager.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateAttesterManager", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.new_attester_manager.is_empty() {
            struct_ser.serialize_field("newAttesterManager", &self.new_attester_manager)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateAttesterManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "new_attester_manager", "newAttesterManager"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            NewAttesterManager,
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
                            "from" => Ok(GeneratedField::From),
                            "newAttesterManager" | "new_attester_manager" => {
                                Ok(GeneratedField::NewAttesterManager)
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
            type Value = MsgUpdateAttesterManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateAttesterManager")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateAttesterManager, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut new_attester_manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAttesterManager => {
                            if new_attester_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newAttesterManager",
                                ));
                            }
                            new_attester_manager__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateAttesterManager {
                    from: from__.unwrap_or_default(),
                    new_attester_manager: new_attester_manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateAttesterManager",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateAttesterManagerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateAttesterManagerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateAttesterManagerResponse {
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
            type Value = MsgUpdateAttesterManagerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateAttesterManagerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateAttesterManagerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateAttesterManagerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateAttesterManagerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateMaxMessageBodySize {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.message_size != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateMaxMessageBodySize", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.message_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "messageSize",
                ToString::to_string(&self.message_size).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateMaxMessageBodySize {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "message_size", "messageSize"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            MessageSize,
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
                            "from" => Ok(GeneratedField::From),
                            "messageSize" | "message_size" => Ok(GeneratedField::MessageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateMaxMessageBodySize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateMaxMessageBodySize")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateMaxMessageBodySize, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut message_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessageSize => {
                            if message_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageSize"));
                            }
                            message_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgUpdateMaxMessageBodySize {
                    from: from__.unwrap_or_default(),
                    message_size: message_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateMaxMessageBodySize",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateMaxMessageBodySizeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgUpdateMaxMessageBodySizeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateMaxMessageBodySizeResponse {
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
            type Value = MsgUpdateMaxMessageBodySizeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateMaxMessageBodySizeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateMaxMessageBodySizeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateMaxMessageBodySizeResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateMaxMessageBodySizeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateOwner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.new_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MsgUpdateOwner", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateOwner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "new_owner", "newOwner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
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
                            "from" => Ok(GeneratedField::From),
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
            type Value = MsgUpdateOwner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateOwner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateOwner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut new_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewOwner => {
                            if new_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newOwner"));
                            }
                            new_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateOwner {
                    from: from__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MsgUpdateOwner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateOwnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateOwnerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateOwnerResponse {
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
            type Value = MsgUpdateOwnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateOwnerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateOwnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateOwnerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateOwnerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdatePauser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.new_pauser.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.MsgUpdatePauser", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.new_pauser.is_empty() {
            struct_ser.serialize_field("newPauser", &self.new_pauser)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdatePauser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "new_pauser", "newPauser"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            NewPauser,
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
                            "from" => Ok(GeneratedField::From),
                            "newPauser" | "new_pauser" => Ok(GeneratedField::NewPauser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdatePauser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdatePauser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdatePauser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut new_pauser__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPauser => {
                            if new_pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPauser"));
                            }
                            new_pauser__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdatePauser {
                    from: from__.unwrap_or_default(),
                    new_pauser: new_pauser__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.MsgUpdatePauser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdatePauserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdatePauserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdatePauserResponse {
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
            type Value = MsgUpdatePauserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdatePauserResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdatePauserResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdatePauserResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdatePauserResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateSignatureThreshold {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateSignatureThreshold", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateSignatureThreshold {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Amount,
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
                            "from" => Ok(GeneratedField::From),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateSignatureThreshold;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateSignatureThreshold")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateSignatureThreshold, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgUpdateSignatureThreshold {
                    from: from__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateSignatureThreshold",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateSignatureThresholdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.MsgUpdateSignatureThresholdResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateSignatureThresholdResponse {
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
            type Value = MsgUpdateSignatureThresholdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateSignatureThresholdResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateSignatureThresholdResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateSignatureThresholdResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateSignatureThresholdResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateTokenController {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.new_token_controller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateTokenController", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.new_token_controller.is_empty() {
            struct_ser.serialize_field("newTokenController", &self.new_token_controller)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateTokenController {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "new_token_controller", "newTokenController"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            NewTokenController,
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
                            "from" => Ok(GeneratedField::From),
                            "newTokenController" | "new_token_controller" => {
                                Ok(GeneratedField::NewTokenController)
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
            type Value = MsgUpdateTokenController;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateTokenController")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateTokenController, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut new_token_controller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewTokenController => {
                            if new_token_controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newTokenController",
                                ));
                            }
                            new_token_controller__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateTokenController {
                    from: from__.unwrap_or_default(),
                    new_token_controller: new_token_controller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateTokenController",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateTokenControllerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.MsgUpdateTokenControllerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateTokenControllerResponse {
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
            type Value = MsgUpdateTokenControllerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.MsgUpdateTokenControllerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateTokenControllerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateTokenControllerResponse {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.MsgUpdateTokenControllerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Nonce {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_domain != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.Nonce", len)?;
        if self.source_domain != 0 {
            struct_ser.serialize_field("sourceDomain", &self.source_domain)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Nonce {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["source_domain", "sourceDomain", "nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceDomain,
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
                            "sourceDomain" | "source_domain" => Ok(GeneratedField::SourceDomain),
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
            type Value = Nonce;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.Nonce")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Nonce, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_domain__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceDomain => {
                            if source_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDomain"));
                            }
                            source_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
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
                Ok(Nonce {
                    source_domain: source_domain__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.Nonce", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OwnerUpdated {
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
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.OwnerUpdated", len)?;
        if !self.previous_owner.is_empty() {
            struct_ser.serialize_field("previousOwner", &self.previous_owner)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OwnerUpdated {
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
            type Value = OwnerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.OwnerUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OwnerUpdated, V::Error>
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
                Ok(OwnerUpdated {
                    previous_owner: previous_owner__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.OwnerUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OwnershipTransferStarted {
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
            serializer.serialize_struct("circle.cctp.v1.OwnershipTransferStarted", len)?;
        if !self.previous_owner.is_empty() {
            struct_ser.serialize_field("previousOwner", &self.previous_owner)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OwnershipTransferStarted {
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
            type Value = OwnershipTransferStarted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.OwnershipTransferStarted")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OwnershipTransferStarted, V::Error>
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
                Ok(OwnershipTransferStarted {
                    previous_owner: previous_owner__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.OwnershipTransferStarted",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PauserUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_pauser.is_empty() {
            len += 1;
        }
        if !self.new_pauser.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.PauserUpdated", len)?;
        if !self.previous_pauser.is_empty() {
            struct_ser.serialize_field("previousPauser", &self.previous_pauser)?;
        }
        if !self.new_pauser.is_empty() {
            struct_ser.serialize_field("newPauser", &self.new_pauser)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PauserUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_pauser",
            "previousPauser",
            "new_pauser",
            "newPauser",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousPauser,
            NewPauser,
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
                            "previousPauser" | "previous_pauser" => {
                                Ok(GeneratedField::PreviousPauser)
                            }
                            "newPauser" | "new_pauser" => Ok(GeneratedField::NewPauser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PauserUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.PauserUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PauserUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_pauser__ = None;
                let mut new_pauser__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousPauser => {
                            if previous_pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousPauser"));
                            }
                            previous_pauser__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPauser => {
                            if new_pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPauser"));
                            }
                            new_pauser__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PauserUpdated {
                    previous_pauser: previous_pauser__.unwrap_or_default(),
                    new_pauser: new_pauser__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.PauserUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PerMessageBurnLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.PerMessageBurnLimit", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PerMessageBurnLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            Amount,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PerMessageBurnLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.PerMessageBurnLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PerMessageBurnLimit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PerMessageBurnLimit {
                    denom: denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.PerMessageBurnLimit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllAttestersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllAttestersRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllAttestersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllAttestersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllAttestersRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllAttestersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllAttestersRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllAttestersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllAttestersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attesters.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllAttestersResponse", len)?;
        if !self.attesters.is_empty() {
            struct_ser.serialize_field("attesters", &self.attesters)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllAttestersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attesters", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attesters,
            Pagination,
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
                            "attesters" => Ok(GeneratedField::Attesters),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllAttestersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllAttestersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllAttestersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attesters__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attesters => {
                            if attesters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attesters"));
                            }
                            attesters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllAttestersResponse {
                    attesters: attesters__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllAttestersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllPerMessageBurnLimitsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryAllPerMessageBurnLimitsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllPerMessageBurnLimitsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllPerMessageBurnLimitsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllPerMessageBurnLimitsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllPerMessageBurnLimitsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllPerMessageBurnLimitsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllPerMessageBurnLimitsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllPerMessageBurnLimitsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.burn_limits.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryAllPerMessageBurnLimitsResponse", len)?;
        if !self.burn_limits.is_empty() {
            struct_ser.serialize_field("burnLimits", &self.burn_limits)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllPerMessageBurnLimitsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["burn_limits", "burnLimits", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BurnLimits,
            Pagination,
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
                            "burnLimits" | "burn_limits" => Ok(GeneratedField::BurnLimits),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllPerMessageBurnLimitsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllPerMessageBurnLimitsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllPerMessageBurnLimitsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut burn_limits__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BurnLimits => {
                            if burn_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnLimits"));
                            }
                            burn_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllPerMessageBurnLimitsResponse {
                    burn_limits: burn_limits__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllPerMessageBurnLimitsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllTokenPairsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllTokenPairsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllTokenPairsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllTokenPairsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllTokenPairsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllTokenPairsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllTokenPairsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllTokenPairsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllTokenPairsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token_pairs.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllTokenPairsResponse", len)?;
        if !self.token_pairs.is_empty() {
            struct_ser.serialize_field("tokenPairs", &self.token_pairs)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllTokenPairsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["token_pairs", "tokenPairs", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenPairs,
            Pagination,
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
                            "tokenPairs" | "token_pairs" => Ok(GeneratedField::TokenPairs),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllTokenPairsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllTokenPairsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllTokenPairsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut token_pairs__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TokenPairs => {
                            if token_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenPairs"));
                            }
                            token_pairs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllTokenPairsResponse {
                    token_pairs: token_pairs__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllTokenPairsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllUsedNoncesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllUsedNoncesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllUsedNoncesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllUsedNoncesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllUsedNoncesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllUsedNoncesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllUsedNoncesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllUsedNoncesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryAllUsedNoncesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.used_nonces.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryAllUsedNoncesResponse", len)?;
        if !self.used_nonces.is_empty() {
            struct_ser.serialize_field("usedNonces", &self.used_nonces)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllUsedNoncesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["used_nonces", "usedNonces", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UsedNonces,
            Pagination,
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
                            "usedNonces" | "used_nonces" => Ok(GeneratedField::UsedNonces),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllUsedNoncesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryAllUsedNoncesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryAllUsedNoncesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut used_nonces__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UsedNonces => {
                            if used_nonces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedNonces"));
                            }
                            used_nonces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllUsedNoncesResponse {
                    used_nonces: used_nonces__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryAllUsedNoncesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBurnMessageVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryBurnMessageVersionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBurnMessageVersionRequest {
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
            type Value = QueryBurnMessageVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryBurnMessageVersionRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBurnMessageVersionRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBurnMessageVersionRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryBurnMessageVersionRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBurnMessageVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryBurnMessageVersionResponse", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBurnMessageVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["version"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBurnMessageVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryBurnMessageVersionResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBurnMessageVersionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryBurnMessageVersionResponse {
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryBurnMessageVersionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetAttesterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attester.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetAttesterRequest", len)?;
        if !self.attester.is_empty() {
            struct_ser.serialize_field("attester", &self.attester)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetAttesterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attester,
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
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetAttesterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetAttesterRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetAttesterRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryGetAttesterRequest {
                    attester: attester__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetAttesterRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetAttesterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attester.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetAttesterResponse", len)?;
        if let Some(v) = self.attester.as_ref() {
            struct_ser.serialize_field("attester", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetAttesterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attester"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attester,
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
                            "attester" => Ok(GeneratedField::Attester),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetAttesterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetAttesterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetAttesterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attester__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attester => {
                            if attester__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attester"));
                            }
                            attester__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetAttesterResponse {
                    attester: attester__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetAttesterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetBurningAndMintingPausedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetBurningAndMintingPausedRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetBurningAndMintingPausedRequest {
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
            type Value = QueryGetBurningAndMintingPausedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetBurningAndMintingPausedRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetBurningAndMintingPausedRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryGetBurningAndMintingPausedRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetBurningAndMintingPausedRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetBurningAndMintingPausedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.paused.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "circle.cctp.v1.QueryGetBurningAndMintingPausedResponse",
            len,
        )?;
        if let Some(v) = self.paused.as_ref() {
            struct_ser.serialize_field("paused", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetBurningAndMintingPausedResponse {
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
            type Value = QueryGetBurningAndMintingPausedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetBurningAndMintingPausedResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetBurningAndMintingPausedResponse, V::Error>
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
                            paused__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetBurningAndMintingPausedResponse { paused: paused__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetBurningAndMintingPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetMaxMessageBodySizeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetMaxMessageBodySizeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetMaxMessageBodySizeRequest {
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
            type Value = QueryGetMaxMessageBodySizeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetMaxMessageBodySizeRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetMaxMessageBodySizeRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryGetMaxMessageBodySizeRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetMaxMessageBodySizeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetMaxMessageBodySizeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetMaxMessageBodySizeResponse", len)?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetMaxMessageBodySizeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetMaxMessageBodySizeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetMaxMessageBodySizeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetMaxMessageBodySizeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetMaxMessageBodySizeResponse { amount: amount__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetMaxMessageBodySizeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetNextAvailableNonceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetNextAvailableNonceRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetNextAvailableNonceRequest {
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
            type Value = QueryGetNextAvailableNonceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetNextAvailableNonceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetNextAvailableNonceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryGetNextAvailableNonceRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetNextAvailableNonceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetNextAvailableNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetNextAvailableNonceResponse", len)?;
        if let Some(v) = self.nonce.as_ref() {
            struct_ser.serialize_field("nonce", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetNextAvailableNonceResponse {
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
            type Value = QueryGetNextAvailableNonceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetNextAvailableNonceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetNextAvailableNonceResponse, V::Error>
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
                            nonce__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetNextAvailableNonceResponse { nonce: nonce__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetNextAvailableNonceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetPerMessageBurnLimitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetPerMessageBurnLimitRequest", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetPerMessageBurnLimitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetPerMessageBurnLimitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetPerMessageBurnLimitRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetPerMessageBurnLimitRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryGetPerMessageBurnLimitRequest {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetPerMessageBurnLimitRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetPerMessageBurnLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.burn_limit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetPerMessageBurnLimitResponse", len)?;
        if let Some(v) = self.burn_limit.as_ref() {
            struct_ser.serialize_field("burnLimit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetPerMessageBurnLimitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["burn_limit", "burnLimit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BurnLimit,
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
                            "burnLimit" | "burn_limit" => Ok(GeneratedField::BurnLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetPerMessageBurnLimitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetPerMessageBurnLimitResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetPerMessageBurnLimitResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut burn_limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BurnLimit => {
                            if burn_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnLimit"));
                            }
                            burn_limit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetPerMessageBurnLimitResponse {
                    burn_limit: burn_limit__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetPerMessageBurnLimitResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetSendingAndReceivingMessagesPausedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedRequest",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetSendingAndReceivingMessagesPausedRequest {
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
            type Value = QueryGetSendingAndReceivingMessagesPausedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedRequest",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetSendingAndReceivingMessagesPausedRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryGetSendingAndReceivingMessagesPausedRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetSendingAndReceivingMessagesPausedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.paused.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedResponse",
            len,
        )?;
        if let Some(v) = self.paused.as_ref() {
            struct_ser.serialize_field("paused", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetSendingAndReceivingMessagesPausedResponse {
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
            type Value = QueryGetSendingAndReceivingMessagesPausedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetSendingAndReceivingMessagesPausedResponse, V::Error>
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
                            paused__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetSendingAndReceivingMessagesPausedResponse { paused: paused__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetSendingAndReceivingMessagesPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetSignatureThresholdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetSignatureThresholdRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetSignatureThresholdRequest {
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
            type Value = QueryGetSignatureThresholdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetSignatureThresholdRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetSignatureThresholdRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryGetSignatureThresholdRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetSignatureThresholdRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetSignatureThresholdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryGetSignatureThresholdResponse", len)?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetSignatureThresholdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetSignatureThresholdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetSignatureThresholdResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetSignatureThresholdResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetSignatureThresholdResponse { amount: amount__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetSignatureThresholdResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetTokenPairRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetTokenPairRequest", len)?;
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            struct_ser.serialize_field("remoteToken", &self.remote_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetTokenPairRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteDomain,
            RemoteToken,
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
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetTokenPairRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetTokenPairRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetTokenPairRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryGetTokenPairRequest {
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetTokenPairRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetTokenPairResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pair.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetTokenPairResponse", len)?;
        if let Some(v) = self.pair.as_ref() {
            struct_ser.serialize_field("pair", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetTokenPairResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pair"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pair,
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
                            "pair" => Ok(GeneratedField::Pair),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryGetTokenPairResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetTokenPairResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetTokenPairResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pair__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pair => {
                            if pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pair"));
                            }
                            pair__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetTokenPairResponse { pair: pair__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetTokenPairResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetUsedNonceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_domain != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetUsedNonceRequest", len)?;
        if self.source_domain != 0 {
            struct_ser.serialize_field("sourceDomain", &self.source_domain)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetUsedNonceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["source_domain", "sourceDomain", "nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceDomain,
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
                            "sourceDomain" | "source_domain" => Ok(GeneratedField::SourceDomain),
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
            type Value = QueryGetUsedNonceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetUsedNonceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetUsedNonceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_domain__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceDomain => {
                            if source_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDomain"));
                            }
                            source_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
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
                Ok(QueryGetUsedNonceRequest {
                    source_domain: source_domain__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetUsedNonceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryGetUsedNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryGetUsedNonceResponse", len)?;
        if let Some(v) = self.nonce.as_ref() {
            struct_ser.serialize_field("nonce", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryGetUsedNonceResponse {
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
            type Value = QueryGetUsedNonceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryGetUsedNonceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryGetUsedNonceResponse, V::Error>
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
                            nonce__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryGetUsedNonceResponse { nonce: nonce__ })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryGetUsedNonceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLocalDomainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryLocalDomainRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLocalDomainRequest {
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
            type Value = QueryLocalDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryLocalDomainRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLocalDomainRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLocalDomainRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryLocalDomainRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLocalDomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryLocalDomainResponse", len)?;
        if self.domain_id != 0 {
            struct_ser.serialize_field("domainId", &self.domain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLocalDomainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["domain_id", "domainId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLocalDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryLocalDomainResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLocalDomainResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryLocalDomainResponse {
                    domain_id: domain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryLocalDomainResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLocalMessageVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryLocalMessageVersionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLocalMessageVersionRequest {
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
            type Value = QueryLocalMessageVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryLocalMessageVersionRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLocalMessageVersionRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLocalMessageVersionRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryLocalMessageVersionRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLocalMessageVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryLocalMessageVersionResponse", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLocalMessageVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["version"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLocalMessageVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryLocalMessageVersionResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLocalMessageVersionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryLocalMessageVersionResponse {
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryLocalMessageVersionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRemoteTokenMessengerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryRemoteTokenMessengerRequest", len)?;
        if self.domain_id != 0 {
            struct_ser.serialize_field("domainId", &self.domain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRemoteTokenMessengerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["domain_id", "domainId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRemoteTokenMessengerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRemoteTokenMessengerRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRemoteTokenMessengerRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRemoteTokenMessengerRequest {
                    domain_id: domain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRemoteTokenMessengerRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRemoteTokenMessengerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_token_messenger.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryRemoteTokenMessengerResponse", len)?;
        if let Some(v) = self.remote_token_messenger.as_ref() {
            struct_ser.serialize_field("remoteTokenMessenger", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRemoteTokenMessengerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["remote_token_messenger", "remoteTokenMessenger"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteTokenMessenger,
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
                            "remoteTokenMessenger" | "remote_token_messenger" => {
                                Ok(GeneratedField::RemoteTokenMessenger)
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
            type Value = QueryRemoteTokenMessengerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRemoteTokenMessengerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRemoteTokenMessengerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remote_token_messenger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteTokenMessenger => {
                            if remote_token_messenger__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "remoteTokenMessenger",
                                ));
                            }
                            remote_token_messenger__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryRemoteTokenMessengerResponse {
                    remote_token_messenger: remote_token_messenger__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRemoteTokenMessengerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRemoteTokenMessengersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryRemoteTokenMessengersRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRemoteTokenMessengersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRemoteTokenMessengersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRemoteTokenMessengersRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRemoteTokenMessengersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryRemoteTokenMessengersRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRemoteTokenMessengersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRemoteTokenMessengersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_token_messengers.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("circle.cctp.v1.QueryRemoteTokenMessengersResponse", len)?;
        if !self.remote_token_messengers.is_empty() {
            struct_ser.serialize_field("remoteTokenMessengers", &self.remote_token_messengers)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRemoteTokenMessengersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_token_messengers",
            "remoteTokenMessengers",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteTokenMessengers,
            Pagination,
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
                            "remoteTokenMessengers" | "remote_token_messengers" => {
                                Ok(GeneratedField::RemoteTokenMessengers)
                            }
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRemoteTokenMessengersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRemoteTokenMessengersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRemoteTokenMessengersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remote_token_messengers__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteTokenMessengers => {
                            if remote_token_messengers__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "remoteTokenMessengers",
                                ));
                            }
                            remote_token_messengers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryRemoteTokenMessengersResponse {
                    remote_token_messengers: remote_token_messengers__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRemoteTokenMessengersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRolesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("circle.cctp.v1.QueryRolesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRolesRequest {
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
            type Value = QueryRolesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRolesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRolesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryRolesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRolesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRolesResponse {
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
        if !self.attester_manager.is_empty() {
            len += 1;
        }
        if !self.pauser.is_empty() {
            len += 1;
        }
        if !self.token_controller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.QueryRolesResponse", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.attester_manager.is_empty() {
            struct_ser.serialize_field("attesterManager", &self.attester_manager)?;
        }
        if !self.pauser.is_empty() {
            struct_ser.serialize_field("pauser", &self.pauser)?;
        }
        if !self.token_controller.is_empty() {
            struct_ser.serialize_field("tokenController", &self.token_controller)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRolesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "attester_manager",
            "attesterManager",
            "pauser",
            "token_controller",
            "tokenController",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            AttesterManager,
            Pauser,
            TokenController,
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
                            "attesterManager" | "attester_manager" => {
                                Ok(GeneratedField::AttesterManager)
                            }
                            "pauser" => Ok(GeneratedField::Pauser),
                            "tokenController" | "token_controller" => {
                                Ok(GeneratedField::TokenController)
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
            type Value = QueryRolesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.QueryRolesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRolesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut attester_manager__ = None;
                let mut pauser__ = None;
                let mut token_controller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttesterManager => {
                            if attester_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attesterManager"));
                            }
                            attester_manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pauser => {
                            if pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauser"));
                            }
                            pauser__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenController => {
                            if token_controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenController"));
                            }
                            token_controller__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRolesResponse {
                    owner: owner__.unwrap_or_default(),
                    attester_manager: attester_manager__.unwrap_or_default(),
                    pauser: pauser__.unwrap_or_default(),
                    token_controller: token_controller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.QueryRolesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RemoteTokenMessenger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain_id != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.RemoteTokenMessenger", len)?;
        if self.domain_id != 0 {
            struct_ser.serialize_field("domainId", &self.domain_id)?;
        }
        if !self.address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "address",
                pbjson::private::base64::encode(&self.address).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteTokenMessenger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["domain_id", "domainId", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DomainId,
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
                            "domainId" | "domain_id" => Ok(GeneratedField::DomainId),
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
            type Value = RemoteTokenMessenger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.RemoteTokenMessenger")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RemoteTokenMessenger, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain_id__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DomainId => {
                            if domain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainId"));
                            }
                            domain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(RemoteTokenMessenger {
                    domain_id: domain_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.RemoteTokenMessenger",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RemoteTokenMessengerAdded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain != 0 {
            len += 1;
        }
        if !self.remote_token_messenger.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.RemoteTokenMessengerAdded", len)?;
        if self.domain != 0 {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.remote_token_messenger.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteTokenMessenger",
                pbjson::private::base64::encode(&self.remote_token_messenger).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteTokenMessengerAdded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["domain", "remote_token_messenger", "remoteTokenMessenger"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            RemoteTokenMessenger,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "remoteTokenMessenger" | "remote_token_messenger" => {
                                Ok(GeneratedField::RemoteTokenMessenger)
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
            type Value = RemoteTokenMessengerAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.RemoteTokenMessengerAdded")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RemoteTokenMessengerAdded, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut remote_token_messenger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteTokenMessenger => {
                            if remote_token_messenger__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "remoteTokenMessenger",
                                ));
                            }
                            remote_token_messenger__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(RemoteTokenMessengerAdded {
                    domain: domain__.unwrap_or_default(),
                    remote_token_messenger: remote_token_messenger__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.RemoteTokenMessengerAdded",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RemoteTokenMessengerRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.domain != 0 {
            len += 1;
        }
        if !self.remote_token_messenger.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.RemoteTokenMessengerRemoved", len)?;
        if self.domain != 0 {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.remote_token_messenger.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteTokenMessenger",
                pbjson::private::base64::encode(&self.remote_token_messenger).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteTokenMessengerRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["domain", "remote_token_messenger", "remoteTokenMessenger"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            RemoteTokenMessenger,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "remoteTokenMessenger" | "remote_token_messenger" => {
                                Ok(GeneratedField::RemoteTokenMessenger)
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
            type Value = RemoteTokenMessengerRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.RemoteTokenMessengerRemoved")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RemoteTokenMessengerRemoved, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut remote_token_messenger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteTokenMessenger => {
                            if remote_token_messenger__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "remoteTokenMessenger",
                                ));
                            }
                            remote_token_messenger__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(RemoteTokenMessengerRemoved {
                    domain: domain__.unwrap_or_default(),
                    remote_token_messenger: remote_token_messenger__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.RemoteTokenMessengerRemoved",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SendingAndReceivingMessagesPaused {
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
            serializer.serialize_struct("circle.cctp.v1.SendingAndReceivingMessagesPaused", len)?;
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendingAndReceivingMessagesPaused {
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
            type Value = SendingAndReceivingMessagesPaused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SendingAndReceivingMessagesPaused")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SendingAndReceivingMessagesPaused, V::Error>
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
                Ok(SendingAndReceivingMessagesPaused {
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SendingAndReceivingMessagesPaused",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SendingAndReceivingPausedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.SendingAndReceivingPausedEvent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendingAndReceivingPausedEvent {
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
            type Value = SendingAndReceivingPausedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SendingAndReceivingPausedEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SendingAndReceivingPausedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SendingAndReceivingPausedEvent {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SendingAndReceivingPausedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SendingAndReceivingUnpausedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("circle.cctp.v1.SendingAndReceivingUnpausedEvent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendingAndReceivingUnpausedEvent {
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
            type Value = SendingAndReceivingUnpausedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SendingAndReceivingUnpausedEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SendingAndReceivingUnpausedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SendingAndReceivingUnpausedEvent {})
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SendingAndReceivingUnpausedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SetBurnLimitPerMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        if !self.burn_limit_per_message.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.SetBurnLimitPerMessage", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if !self.burn_limit_per_message.is_empty() {
            struct_ser.serialize_field("burnLimitPerMessage", &self.burn_limit_per_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetBurnLimitPerMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["token", "burn_limit_per_message", "burnLimitPerMessage"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            BurnLimitPerMessage,
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
                            "token" => Ok(GeneratedField::Token),
                            "burnLimitPerMessage" | "burn_limit_per_message" => {
                                Ok(GeneratedField::BurnLimitPerMessage)
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
            type Value = SetBurnLimitPerMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SetBurnLimitPerMessage")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetBurnLimitPerMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut burn_limit_per_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BurnLimitPerMessage => {
                            if burn_limit_per_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "burnLimitPerMessage",
                                ));
                            }
                            burn_limit_per_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetBurnLimitPerMessage {
                    token: token__.unwrap_or_default(),
                    burn_limit_per_message: burn_limit_per_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SetBurnLimitPerMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SignatureThreshold {
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
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.SignatureThreshold", len)?;
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignatureThreshold {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignatureThreshold;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SignatureThreshold")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SignatureThreshold, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
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
                    }
                }
                Ok(SignatureThreshold {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SignatureThreshold",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SignatureThresholdUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.old_signature_threshold != 0 {
            len += 1;
        }
        if self.new_signature_threshold != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.SignatureThresholdUpdated", len)?;
        if self.old_signature_threshold != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "oldSignatureThreshold",
                ToString::to_string(&self.old_signature_threshold).as_str(),
            )?;
        }
        if self.new_signature_threshold != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "newSignatureThreshold",
                ToString::to_string(&self.new_signature_threshold).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignatureThresholdUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "old_signature_threshold",
            "oldSignatureThreshold",
            "new_signature_threshold",
            "newSignatureThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OldSignatureThreshold,
            NewSignatureThreshold,
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
                            "oldSignatureThreshold" | "old_signature_threshold" => {
                                Ok(GeneratedField::OldSignatureThreshold)
                            }
                            "newSignatureThreshold" | "new_signature_threshold" => {
                                Ok(GeneratedField::NewSignatureThreshold)
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
            type Value = SignatureThresholdUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.SignatureThresholdUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SignatureThresholdUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut old_signature_threshold__ = None;
                let mut new_signature_threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OldSignatureThreshold => {
                            if old_signature_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "oldSignatureThreshold",
                                ));
                            }
                            old_signature_threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewSignatureThreshold => {
                            if new_signature_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newSignatureThreshold",
                                ));
                            }
                            new_signature_threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SignatureThresholdUpdated {
                    old_signature_threshold: old_signature_threshold__.unwrap_or_default(),
                    new_signature_threshold: new_signature_threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.SignatureThresholdUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TokenControllerUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_token_controller.is_empty() {
            len += 1;
        }
        if !self.new_token_controller.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.TokenControllerUpdated", len)?;
        if !self.previous_token_controller.is_empty() {
            struct_ser
                .serialize_field("previousTokenController", &self.previous_token_controller)?;
        }
        if !self.new_token_controller.is_empty() {
            struct_ser.serialize_field("newTokenController", &self.new_token_controller)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenControllerUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_token_controller",
            "previousTokenController",
            "new_token_controller",
            "newTokenController",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousTokenController,
            NewTokenController,
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
                            "previousTokenController" | "previous_token_controller" => {
                                Ok(GeneratedField::PreviousTokenController)
                            }
                            "newTokenController" | "new_token_controller" => {
                                Ok(GeneratedField::NewTokenController)
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
            type Value = TokenControllerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.TokenControllerUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TokenControllerUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_token_controller__ = None;
                let mut new_token_controller__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousTokenController => {
                            if previous_token_controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "previousTokenController",
                                ));
                            }
                            previous_token_controller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewTokenController => {
                            if new_token_controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "newTokenController",
                                ));
                            }
                            new_token_controller__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TokenControllerUpdated {
                    previous_token_controller: previous_token_controller__.unwrap_or_default(),
                    new_token_controller: new_token_controller__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.TokenControllerUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TokenPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        if !self.local_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.TokenPair", len)?;
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteToken",
                pbjson::private::base64::encode(&self.remote_token).as_str(),
            )?;
        }
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
            "local_token",
            "localToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteDomain,
            RemoteToken,
            LocalToken,
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
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.TokenPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TokenPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                let mut local_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TokenPair {
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                    local_token: local_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.TokenPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenPairLinked {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_token.is_empty() {
            len += 1;
        }
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("circle.cctp.v1.TokenPairLinked", len)?;
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteToken",
                pbjson::private::base64::encode(&self.remote_token).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenPairLinked {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_token",
            "localToken",
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalToken,
            RemoteDomain,
            RemoteToken,
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
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenPairLinked;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.TokenPairLinked")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TokenPairLinked, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut local_token__ = None;
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TokenPairLinked {
                    local_token: local_token__.unwrap_or_default(),
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("circle.cctp.v1.TokenPairLinked", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenPairUnlinked {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_token.is_empty() {
            len += 1;
        }
        if self.remote_domain != 0 {
            len += 1;
        }
        if !self.remote_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("circle.cctp.v1.TokenPairUnlinked", len)?;
        if !self.local_token.is_empty() {
            struct_ser.serialize_field("localToken", &self.local_token)?;
        }
        if self.remote_domain != 0 {
            struct_ser.serialize_field("remoteDomain", &self.remote_domain)?;
        }
        if !self.remote_token.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "remoteToken",
                pbjson::private::base64::encode(&self.remote_token).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenPairUnlinked {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_token",
            "localToken",
            "remote_domain",
            "remoteDomain",
            "remote_token",
            "remoteToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalToken,
            RemoteDomain,
            RemoteToken,
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
                            "localToken" | "local_token" => Ok(GeneratedField::LocalToken),
                            "remoteDomain" | "remote_domain" => Ok(GeneratedField::RemoteDomain),
                            "remoteToken" | "remote_token" => Ok(GeneratedField::RemoteToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenPairUnlinked;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct circle.cctp.v1.TokenPairUnlinked")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TokenPairUnlinked, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut local_token__ = None;
                let mut remote_domain__ = None;
                let mut remote_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalToken => {
                            if local_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localToken"));
                            }
                            local_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteDomain => {
                            if remote_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteDomain"));
                            }
                            remote_domain__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RemoteToken => {
                            if remote_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteToken"));
                            }
                            remote_token__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TokenPairUnlinked {
                    local_token: local_token__.unwrap_or_default(),
                    remote_domain: remote_domain__.unwrap_or_default(),
                    remote_token: remote_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "circle.cctp.v1.TokenPairUnlinked",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
