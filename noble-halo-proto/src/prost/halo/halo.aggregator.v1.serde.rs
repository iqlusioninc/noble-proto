// @generated
impl serde::Serialize for BalanceReported {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if !self.interest.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.BalanceReported", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if !self.interest.is_empty() {
            struct_ser.serialize_field("interest", &self.interest)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BalanceReported {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round_id",
            "roundId",
            "balance",
            "interest",
            "price",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
            Balance,
            Interest,
            Price,
            UpdatedAt,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            "balance" => Ok(GeneratedField::Balance),
                            "interest" => Ok(GeneratedField::Interest),
                            "price" => Ok(GeneratedField::Price),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BalanceReported;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.BalanceReported")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BalanceReported, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                let mut balance__ = None;
                let mut interest__ = None;
                let mut price__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BalanceReported {
                    round_id: round_id__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.BalanceReported",
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
        if self.last_round_id != 0 {
            len += 1;
        }
        if !self.next_price.is_empty() {
            len += 1;
        }
        if !self.rounds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("halo.aggregator.v1.GenesisState", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.last_round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "lastRoundId",
                ToString::to_string(&self.last_round_id).as_str(),
            )?;
        }
        if !self.next_price.is_empty() {
            struct_ser.serialize_field("nextPrice", &self.next_price)?;
        }
        if !self.rounds.is_empty() {
            struct_ser.serialize_field("rounds", &self.rounds)?;
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
            "last_round_id",
            "lastRoundId",
            "next_price",
            "nextPrice",
            "rounds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            LastRoundId,
            NextPrice,
            Rounds,
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
                            "lastRoundId" | "last_round_id" => Ok(GeneratedField::LastRoundId),
                            "nextPrice" | "next_price" => Ok(GeneratedField::NextPrice),
                            "rounds" => Ok(GeneratedField::Rounds),
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
                formatter.write_str("struct halo.aggregator.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut last_round_id__ = None;
                let mut next_price__ = None;
                let mut rounds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastRoundId => {
                            if last_round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastRoundId"));
                            }
                            last_round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NextPrice => {
                            if next_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPrice"));
                            }
                            next_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rounds => {
                            if rounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rounds"));
                            }
                            rounds__ = Some(
                                map_.next_value::<std::collections::HashMap<
                                    ::pbjson::private::NumberDeserialize<u64>,
                                    _,
                                >>()?
                                .into_iter()
                                .map(|(k, v)| (k.0, v))
                                .collect(),
                            );
                        }
                    }
                }
                Ok(GenesisState {
                    owner: owner__.unwrap_or_default(),
                    last_round_id: last_round_id__.unwrap_or_default(),
                    next_price: next_price__.unwrap_or_default(),
                    rounds: rounds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.aggregator.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgReportBalance {
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
        if !self.principal.is_empty() {
            len += 1;
        }
        if !self.interest.is_empty() {
            len += 1;
        }
        if !self.total_supply.is_empty() {
            len += 1;
        }
        if !self.next_price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.MsgReportBalance", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if !self.interest.is_empty() {
            struct_ser.serialize_field("interest", &self.interest)?;
        }
        if !self.total_supply.is_empty() {
            struct_ser.serialize_field("totalSupply", &self.total_supply)?;
        }
        if !self.next_price.is_empty() {
            struct_ser.serialize_field("nextPrice", &self.next_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReportBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signer",
            "principal",
            "interest",
            "total_supply",
            "totalSupply",
            "next_price",
            "nextPrice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Principal,
            Interest,
            TotalSupply,
            NextPrice,
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
                            "principal" => Ok(GeneratedField::Principal),
                            "interest" => Ok(GeneratedField::Interest),
                            "totalSupply" | "total_supply" => Ok(GeneratedField::TotalSupply),
                            "nextPrice" | "next_price" => Ok(GeneratedField::NextPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgReportBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.MsgReportBalance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgReportBalance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut principal__ = None;
                let mut interest__ = None;
                let mut total_supply__ = None;
                let mut next_price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalSupply => {
                            if total_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSupply"));
                            }
                            total_supply__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPrice => {
                            if next_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPrice"));
                            }
                            next_price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgReportBalance {
                    signer: signer__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    total_supply: total_supply__.unwrap_or_default(),
                    next_price: next_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.MsgReportBalance",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgReportBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.MsgReportBalanceResponse", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReportBalanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["round_id", "roundId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgReportBalanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.MsgReportBalanceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgReportBalanceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgReportBalanceResponse {
                    round_id: round_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.MsgReportBalanceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetNextPrice {
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
        if !self.next_price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.MsgSetNextPrice", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.next_price.is_empty() {
            struct_ser.serialize_field("nextPrice", &self.next_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetNextPrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "next_price", "nextPrice"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            NextPrice,
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
                            "nextPrice" | "next_price" => Ok(GeneratedField::NextPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetNextPrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.MsgSetNextPrice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetNextPrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut next_price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPrice => {
                            if next_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPrice"));
                            }
                            next_price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetNextPrice {
                    signer: signer__.unwrap_or_default(),
                    next_price: next_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.MsgSetNextPrice",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetNextPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.MsgSetNextPriceResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetNextPriceResponse {
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
            type Value = MsgSetNextPriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.MsgSetNextPriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetNextPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetNextPriceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.MsgSetNextPriceResponse",
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
            serializer.serialize_struct("halo.aggregator.v1.MsgTransferOwnership", len)?;
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
                formatter.write_str("struct halo.aggregator.v1.MsgTransferOwnership")
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
            "halo.aggregator.v1.MsgTransferOwnership",
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
        let struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.MsgTransferOwnershipResponse", len)?;
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
                formatter.write_str("struct halo.aggregator.v1.MsgTransferOwnershipResponse")
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
            "halo.aggregator.v1.MsgTransferOwnershipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NextPriceReported {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.NextPriceReported", len)?;
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NextPriceReported {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
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
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NextPriceReported;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.NextPriceReported")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NextPriceReported, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NextPriceReported {
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.NextPriceReported",
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
            serializer.serialize_struct("halo.aggregator.v1.OwnershipTransferred", len)?;
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
                formatter.write_str("struct halo.aggregator.v1.OwnershipTransferred")
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
            "halo.aggregator.v1.OwnershipTransferred",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLatestRoundData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryLatestRoundData", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLatestRoundData {
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
            type Value = QueryLatestRoundData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryLatestRoundData")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestRoundData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLatestRoundData {})
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryLatestRoundData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryLatestRoundDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryLatestRoundDetails", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLatestRoundDetails {
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
            type Value = QueryLatestRoundDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryLatestRoundDetails")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestRoundDetails, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLatestRoundDetails {})
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryLatestRoundDetails",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryNextPrice {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("halo.aggregator.v1.QueryNextPrice", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryNextPrice {
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
            type Value = QueryNextPrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryNextPrice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryNextPrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryNextPrice {})
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryNextPrice",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryNextPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.next_price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryNextPriceResponse", len)?;
        if !self.next_price.is_empty() {
            struct_ser.serialize_field("nextPrice", &self.next_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryNextPriceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["next_price", "nextPrice"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NextPrice,
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
                            "nextPrice" | "next_price" => Ok(GeneratedField::NextPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNextPriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryNextPriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryNextPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut next_price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NextPrice => {
                            if next_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPrice"));
                            }
                            next_price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryNextPriceResponse {
                    next_price: next_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryNextPriceResponse",
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
        let struct_ser = serializer.serialize_struct("halo.aggregator.v1.QueryOwner", len)?;
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
                formatter.write_str("struct halo.aggregator.v1.QueryOwner")
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
        deserializer.deserialize_struct("halo.aggregator.v1.QueryOwner", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("halo.aggregator.v1.QueryOwnerResponse", len)?;
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
                formatter.write_str("struct halo.aggregator.v1.QueryOwnerResponse")
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
            "halo.aggregator.v1.QueryOwnerResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoundData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryRoundData", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoundData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["round_id", "roundId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRoundData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryRoundData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRoundData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRoundData {
                    round_id: round_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryRoundData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoundDataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        if !self.answer.is_empty() {
            len += 1;
        }
        if self.started_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        if self.answered_in_round != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryRoundDataResponse", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if self.started_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("startedAt", ToString::to_string(&self.started_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        if self.answered_in_round != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "answeredInRound",
                ToString::to_string(&self.answered_in_round).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoundDataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round_id",
            "roundId",
            "answer",
            "started_at",
            "startedAt",
            "updated_at",
            "updatedAt",
            "answered_in_round",
            "answeredInRound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
            Answer,
            StartedAt,
            UpdatedAt,
            AnsweredInRound,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            "answer" => Ok(GeneratedField::Answer),
                            "startedAt" | "started_at" => Ok(GeneratedField::StartedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "answeredInRound" | "answered_in_round" => {
                                Ok(GeneratedField::AnsweredInRound)
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
            type Value = QueryRoundDataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryRoundDataResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRoundDataResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                let mut answer__ = None;
                let mut started_at__ = None;
                let mut updated_at__ = None;
                let mut answered_in_round__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartedAt => {
                            if started_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startedAt"));
                            }
                            started_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AnsweredInRound => {
                            if answered_in_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answeredInRound"));
                            }
                            answered_in_round__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRoundDataResponse {
                    round_id: round_id__.unwrap_or_default(),
                    answer: answer__.unwrap_or_default(),
                    started_at: started_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    answered_in_round: answered_in_round__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryRoundDataResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoundDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryRoundDetails", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoundDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["round_id", "roundId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRoundDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryRoundDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRoundDetails, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRoundDetails {
                    round_id: round_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryRoundDetails",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoundDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round_id != 0 {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if !self.interest.is_empty() {
            len += 1;
        }
        if !self.total_supply.is_empty() {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.aggregator.v1.QueryRoundDetailsResponse", len)?;
        if self.round_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("roundId", ToString::to_string(&self.round_id).as_str())?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if !self.interest.is_empty() {
            struct_ser.serialize_field("interest", &self.interest)?;
        }
        if !self.total_supply.is_empty() {
            struct_ser.serialize_field("totalSupply", &self.total_supply)?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoundDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round_id",
            "roundId",
            "balance",
            "interest",
            "total_supply",
            "totalSupply",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
            Balance,
            Interest,
            TotalSupply,
            UpdatedAt,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            "balance" => Ok(GeneratedField::Balance),
                            "interest" => Ok(GeneratedField::Interest),
                            "totalSupply" | "total_supply" => Ok(GeneratedField::TotalSupply),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRoundDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.QueryRoundDetailsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRoundDetailsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                let mut balance__ = None;
                let mut interest__ = None;
                let mut total_supply__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalSupply => {
                            if total_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSupply"));
                            }
                            total_supply__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRoundDetailsResponse {
                    round_id: round_id__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    total_supply: total_supply__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.aggregator.v1.QueryRoundDetailsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RoundData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.answer.is_empty() {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if !self.interest.is_empty() {
            len += 1;
        }
        if !self.supply.is_empty() {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("halo.aggregator.v1.RoundData", len)?;
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if !self.interest.is_empty() {
            struct_ser.serialize_field("interest", &self.interest)?;
        }
        if !self.supply.is_empty() {
            struct_ser.serialize_field("supply", &self.supply)?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoundData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "answer",
            "balance",
            "interest",
            "supply",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Answer,
            Balance,
            Interest,
            Supply,
            UpdatedAt,
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
                            "answer" => Ok(GeneratedField::Answer),
                            "balance" => Ok(GeneratedField::Balance),
                            "interest" => Ok(GeneratedField::Interest),
                            "supply" => Ok(GeneratedField::Supply),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoundData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.aggregator.v1.RoundData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoundData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut answer__ = None;
                let mut balance__ = None;
                let mut interest__ = None;
                let mut supply__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Supply => {
                            if supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supply"));
                            }
                            supply__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(RoundData {
                    answer: answer__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    supply: supply__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.aggregator.v1.RoundData", FIELDS, GeneratedVisitor)
    }
}
