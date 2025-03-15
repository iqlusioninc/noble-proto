// @generated
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.positions.is_empty() {
            len += 1;
        }
        if !self.rewards.is_empty() {
            len += 1;
        }
        if !self.total_flexible_principal.is_empty() {
            len += 1;
        }
        if self.paused != 0 {
            len += 1;
        }
        if self.stats.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.GenesisState", len)?;
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        if !self.total_flexible_principal.is_empty() {
            struct_ser.serialize_field("totalFlexiblePrincipal", &self.total_flexible_principal)?;
        }
        if self.paused != 0 {
            let v = PausedType::try_from(self.paused).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.paused))
            })?;
            struct_ser.serialize_field("paused", &v)?;
        }
        if let Some(v) = self.stats.as_ref() {
            struct_ser.serialize_field("stats", v)?;
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
            "positions",
            "rewards",
            "total_flexible_principal",
            "totalFlexiblePrincipal",
            "paused",
            "stats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Positions,
            Rewards,
            TotalFlexiblePrincipal,
            Paused,
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
                            "positions" => Ok(GeneratedField::Positions),
                            "rewards" => Ok(GeneratedField::Rewards),
                            "totalFlexiblePrincipal" | "total_flexible_principal" => {
                                Ok(GeneratedField::TotalFlexiblePrincipal)
                            }
                            "paused" => Ok(GeneratedField::Paused),
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
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut positions__ = None;
                let mut rewards__ = None;
                let mut total_flexible_principal__ = None;
                let mut paused__ = None;
                let mut stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalFlexiblePrincipal => {
                            if total_flexible_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "totalFlexiblePrincipal",
                                ));
                            }
                            total_flexible_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value::<PausedType>()? as i32);
                        }
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenesisState {
                    positions: positions__.unwrap_or_default(),
                    rewards: rewards__.unwrap_or_default(),
                    total_flexible_principal: total_flexible_principal__.unwrap_or_default(),
                    paused: paused__.unwrap_or_default(),
                    stats: stats__,
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLock {
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
        if self.vault != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.MsgLock", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.vault != 0 {
            let v = VaultType::try_from(self.vault).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.vault))
            })?;
            struct_ser.serialize_field("vault", &v)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "vault", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Vault,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "vault" => Ok(GeneratedField::Vault),
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
            type Value = MsgLock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.MsgLock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgLock, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut vault__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vault => {
                            if vault__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vault"));
                            }
                            vault__ = Some(map_.next_value::<VaultType>()? as i32);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgLock {
                    signer: signer__.unwrap_or_default(),
                    vault: vault__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.vaults.v1.MsgLock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgLockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.MsgLockResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLockResponse {
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
            type Value = MsgLockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.MsgLockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgLockResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLockResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.MsgLockResponse",
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
        if self.paused != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.MsgSetPausedState", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.paused != 0 {
            let v = PausedType::try_from(self.paused).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.paused))
            })?;
            struct_ser.serialize_field("paused", &v)?;
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
                formatter.write_str("struct noble.dollar.vaults.v1.MsgSetPausedState")
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
                            paused__ = Some(map_.next_value::<PausedType>()? as i32);
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
            "noble.dollar.vaults.v1.MsgSetPausedState",
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
            serializer.serialize_struct("noble.dollar.vaults.v1.MsgSetPausedStateResponse", len)?;
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
                formatter.write_str("struct noble.dollar.vaults.v1.MsgSetPausedStateResponse")
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
            "noble.dollar.vaults.v1.MsgSetPausedStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlock {
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
        if self.vault != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.MsgUnlock", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.vault != 0 {
            let v = VaultType::try_from(self.vault).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.vault))
            })?;
            struct_ser.serialize_field("vault", &v)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "vault", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Vault,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "vault" => Ok(GeneratedField::Vault),
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
            type Value = MsgUnlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.MsgUnlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnlock, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut vault__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vault => {
                            if vault__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vault"));
                            }
                            vault__ = Some(map_.next_value::<VaultType>()? as i32);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUnlock {
                    signer: signer__.unwrap_or_default(),
                    vault: vault__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.MsgUnlock",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.MsgUnlockResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlockResponse {
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
            type Value = MsgUnlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.MsgUnlockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnlockResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnlockResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.MsgUnlockResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PausedStateUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.paused.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.PausedStateUpdated", len)?;
        if !self.paused.is_empty() {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PausedStateUpdated {
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
            type Value = PausedStateUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.PausedStateUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PausedStateUpdated, V::Error>
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
                Ok(PausedStateUpdated {
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.PausedStateUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PausedType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Lock => "LOCK",
            Self::Unlock => "UNLOCK",
            Self::All => "ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PausedType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["NONE", "LOCK", "UNLOCK", "ALL"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PausedType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(PausedType::None),
                    "LOCK" => Ok(PausedType::Lock),
                    "UNLOCK" => Ok(PausedType::Unlock),
                    "ALL" => Ok(PausedType::All),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Position {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.Position", len)?;
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["principal", "index", "amount", "time"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Principal,
            Index,
            Amount,
            Time,
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
                            "principal" => Ok(GeneratedField::Principal),
                            "index" => Ok(GeneratedField::Index),
                            "amount" => Ok(GeneratedField::Amount),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut principal__ = None;
                let mut index__ = None;
                let mut amount__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Position {
                    principal: principal__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.vaults.v1.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PositionEntry {
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
        if self.vault != 0 {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.PositionEntry", len)?;
        if !self.address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "address",
                pbjson::private::base64::encode(&self.address).as_str(),
            )?;
        }
        if self.vault != 0 {
            let v = VaultType::try_from(self.vault).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.vault))
            })?;
            struct_ser.serialize_field("vault", &v)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PositionEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "vault", "principal", "index", "amount", "time"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Vault,
            Principal,
            Index,
            Amount,
            Time,
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
                            "vault" => Ok(GeneratedField::Vault),
                            "principal" => Ok(GeneratedField::Principal),
                            "index" => Ok(GeneratedField::Index),
                            "amount" => Ok(GeneratedField::Amount),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PositionEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.PositionEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PositionEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut vault__ = None;
                let mut principal__ = None;
                let mut index__ = None;
                let mut amount__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Vault => {
                            if vault__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vault"));
                            }
                            vault__ = Some(map_.next_value::<VaultType>()? as i32);
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PositionEntry {
                    address: address__.unwrap_or_default(),
                    vault: vault__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.PositionEntry",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PositionLocked {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.vault_type.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.PositionLocked", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.vault_type.is_empty() {
            struct_ser.serialize_field("vaultType", &self.vault_type)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PositionLocked {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
            "vault_type",
            "vaultType",
            "index",
            "amount",
            "principal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            VaultType,
            Index,
            Amount,
            Principal,
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
                            "account" => Ok(GeneratedField::Account),
                            "vaultType" | "vault_type" => Ok(GeneratedField::VaultType),
                            "index" => Ok(GeneratedField::Index),
                            "amount" => Ok(GeneratedField::Amount),
                            "principal" => Ok(GeneratedField::Principal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PositionLocked;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.PositionLocked")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PositionLocked, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut vault_type__ = None;
                let mut index__ = None;
                let mut amount__ = None;
                let mut principal__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VaultType => {
                            if vault_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultType"));
                            }
                            vault_type__ = Some(map_.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PositionLocked {
                    account: account__.unwrap_or_default(),
                    vault_type: vault_type__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.PositionLocked",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PositionUnlocked {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.vault_type.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.PositionUnlocked", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.vault_type.is_empty() {
            struct_ser.serialize_field("vaultType", &self.vault_type)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PositionUnlocked {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
            "vault_type",
            "vaultType",
            "index",
            "amount",
            "principal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            VaultType,
            Index,
            Amount,
            Principal,
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
                            "account" => Ok(GeneratedField::Account),
                            "vaultType" | "vault_type" => Ok(GeneratedField::VaultType),
                            "index" => Ok(GeneratedField::Index),
                            "amount" => Ok(GeneratedField::Amount),
                            "principal" => Ok(GeneratedField::Principal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PositionUnlocked;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.PositionUnlocked")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PositionUnlocked, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut vault_type__ = None;
                let mut index__ = None;
                let mut amount__ = None;
                let mut principal__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VaultType => {
                            if vault_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultType"));
                            }
                            vault_type__ = Some(map_.next_value()?);
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PositionUnlocked {
                    account: account__.unwrap_or_default(),
                    vault_type: vault_type__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.PositionUnlocked",
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
        let struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.QueryPaused", len)?;
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
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPaused")
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
            "noble.dollar.vaults.v1.QueryPaused",
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
        if self.paused != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.QueryPausedResponse", len)?;
        if self.paused != 0 {
            let v = PausedType::try_from(self.paused).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.paused))
            })?;
            struct_ser.serialize_field("paused", &v)?;
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
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPausedResponse")
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
                            paused__ = Some(map_.next_value::<PausedType>()? as i32);
                        }
                    }
                }
                Ok(QueryPausedResponse {
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPendingRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.QueryPendingRewards", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingRewards {
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
            type Value = QueryPendingRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPendingRewards")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPendingRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPendingRewards {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPendingRewards",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPendingRewardsByProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("noble.dollar.vaults.v1.QueryPendingRewardsByProvider", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingRewardsByProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPendingRewardsByProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPendingRewardsByProvider")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingRewardsByProvider, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingRewardsByProvider {
                    provider: provider__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPendingRewardsByProvider",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPendingRewardsByProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pending_rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "noble.dollar.vaults.v1.QueryPendingRewardsByProviderResponse",
            len,
        )?;
        if !self.pending_rewards.is_empty() {
            struct_ser.serialize_field("pendingRewards", &self.pending_rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingRewardsByProviderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pending_rewards", "pendingRewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PendingRewards,
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
                            "pendingRewards" | "pending_rewards" => {
                                Ok(GeneratedField::PendingRewards)
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
            type Value = QueryPendingRewardsByProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct noble.dollar.vaults.v1.QueryPendingRewardsByProviderResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingRewardsByProviderResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pending_rewards__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PendingRewards => {
                            if pending_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingRewards"));
                            }
                            pending_rewards__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingRewardsByProviderResponse {
                    pending_rewards: pending_rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPendingRewardsByProviderResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPendingRewardsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pending_rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("noble.dollar.vaults.v1.QueryPendingRewardsResponse", len)?;
        if !self.pending_rewards.is_empty() {
            struct_ser.serialize_field("pendingRewards", &self.pending_rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingRewardsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pending_rewards", "pendingRewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PendingRewards,
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
                            "pendingRewards" | "pending_rewards" => {
                                Ok(GeneratedField::PendingRewards)
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
            type Value = QueryPendingRewardsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPendingRewardsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingRewardsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pending_rewards__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PendingRewards => {
                            if pending_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingRewards"));
                            }
                            pending_rewards__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingRewardsResponse {
                    pending_rewards: pending_rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPendingRewardsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPositionsByProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.QueryPositionsByProvider", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPositionsByProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPositionsByProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.QueryPositionsByProvider")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPositionsByProvider, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPositionsByProvider {
                    provider: provider__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPositionsByProvider",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPositionsByProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.positions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "noble.dollar.vaults.v1.QueryPositionsByProviderResponse",
            len,
        )?;
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPositionsByProviderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["positions"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Positions,
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
                            "positions" => Ok(GeneratedField::Positions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPositionsByProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct noble.dollar.vaults.v1.QueryPositionsByProviderResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPositionsByProviderResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut positions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPositionsByProviderResponse {
                    positions: positions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryPositionsByProviderResponse",
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
        let struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.QueryStats", len)?;
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
                formatter.write_str("struct noble.dollar.vaults.v1.QueryStats")
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
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryStats",
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
        if !self.flexible_total_principal.is_empty() {
            len += 1;
        }
        if self.flexible_total_users != 0 {
            len += 1;
        }
        if !self.flexible_total_distributed_rewards_principal.is_empty() {
            len += 1;
        }
        if !self.staked_total_principal.is_empty() {
            len += 1;
        }
        if self.staked_total_users != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.QueryStatsResponse", len)?;
        if !self.flexible_total_principal.is_empty() {
            struct_ser.serialize_field("flexibleTotalPrincipal", &self.flexible_total_principal)?;
        }
        if self.flexible_total_users != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "flexibleTotalUsers",
                ToString::to_string(&self.flexible_total_users).as_str(),
            )?;
        }
        if !self.flexible_total_distributed_rewards_principal.is_empty() {
            struct_ser.serialize_field(
                "flexibleTotalDistributedRewardsPrincipal",
                &self.flexible_total_distributed_rewards_principal,
            )?;
        }
        if !self.staked_total_principal.is_empty() {
            struct_ser.serialize_field("stakedTotalPrincipal", &self.staked_total_principal)?;
        }
        if self.staked_total_users != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "stakedTotalUsers",
                ToString::to_string(&self.staked_total_users).as_str(),
            )?;
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
        const FIELDS: &[&str] = &[
            "flexible_total_principal",
            "flexibleTotalPrincipal",
            "flexible_total_users",
            "flexibleTotalUsers",
            "flexible_total_distributed_rewards_principal",
            "flexibleTotalDistributedRewardsPrincipal",
            "staked_total_principal",
            "stakedTotalPrincipal",
            "staked_total_users",
            "stakedTotalUsers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FlexibleTotalPrincipal,
            FlexibleTotalUsers,
            FlexibleTotalDistributedRewardsPrincipal,
            StakedTotalPrincipal,
            StakedTotalUsers,
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
                            "flexibleTotalPrincipal" | "flexible_total_principal" => {
                                Ok(GeneratedField::FlexibleTotalPrincipal)
                            }
                            "flexibleTotalUsers" | "flexible_total_users" => {
                                Ok(GeneratedField::FlexibleTotalUsers)
                            }
                            "flexibleTotalDistributedRewardsPrincipal"
                            | "flexible_total_distributed_rewards_principal" => {
                                Ok(GeneratedField::FlexibleTotalDistributedRewardsPrincipal)
                            }
                            "stakedTotalPrincipal" | "staked_total_principal" => {
                                Ok(GeneratedField::StakedTotalPrincipal)
                            }
                            "stakedTotalUsers" | "staked_total_users" => {
                                Ok(GeneratedField::StakedTotalUsers)
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
            type Value = QueryStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.QueryStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStatsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut flexible_total_principal__ = None;
                let mut flexible_total_users__ = None;
                let mut flexible_total_distributed_rewards_principal__ = None;
                let mut staked_total_principal__ = None;
                let mut staked_total_users__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FlexibleTotalPrincipal => {
                            if flexible_total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalPrincipal",
                                ));
                            }
                            flexible_total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FlexibleTotalUsers => {
                            if flexible_total_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalUsers",
                                ));
                            }
                            flexible_total_users__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FlexibleTotalDistributedRewardsPrincipal => {
                            if flexible_total_distributed_rewards_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalDistributedRewardsPrincipal",
                                ));
                            }
                            flexible_total_distributed_rewards_principal__ =
                                Some(map_.next_value()?);
                        }
                        GeneratedField::StakedTotalPrincipal => {
                            if staked_total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "stakedTotalPrincipal",
                                ));
                            }
                            staked_total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StakedTotalUsers => {
                            if staked_total_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakedTotalUsers"));
                            }
                            staked_total_users__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryStatsResponse {
                    flexible_total_principal: flexible_total_principal__.unwrap_or_default(),
                    flexible_total_users: flexible_total_users__.unwrap_or_default(),
                    flexible_total_distributed_rewards_principal:
                        flexible_total_distributed_rewards_principal__.unwrap_or_default(),
                    staked_total_principal: staked_total_principal__.unwrap_or_default(),
                    staked_total_users: staked_total_users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.QueryStatsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Reward {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        if !self.total.is_empty() {
            len += 1;
        }
        if !self.rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.Reward", len)?;
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.total.is_empty() {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reward {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["index", "total", "rewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            Total,
            Rewards,
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
                            "index" => Ok(GeneratedField::Index),
                            "total" => Ok(GeneratedField::Total),
                            "rewards" => Ok(GeneratedField::Rewards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reward;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.Reward")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Reward, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut total__ = None;
                let mut rewards__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Reward {
                    index: index__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    rewards: rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.vaults.v1.Reward", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RewardClaimed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.vaults.v1.RewardClaimed", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RewardClaimed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["account", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
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
            type Value = RewardClaimed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.vaults.v1.RewardClaimed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RewardClaimed, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RewardClaimed {
                    account: account__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.vaults.v1.RewardClaimed",
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
        if !self.flexible_total_principal.is_empty() {
            len += 1;
        }
        if self.flexible_total_users != 0 {
            len += 1;
        }
        if !self.flexible_total_distributed_rewards_principal.is_empty() {
            len += 1;
        }
        if !self.staked_total_principal.is_empty() {
            len += 1;
        }
        if self.staked_total_users != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.vaults.v1.Stats", len)?;
        if !self.flexible_total_principal.is_empty() {
            struct_ser.serialize_field("flexibleTotalPrincipal", &self.flexible_total_principal)?;
        }
        if self.flexible_total_users != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "flexibleTotalUsers",
                ToString::to_string(&self.flexible_total_users).as_str(),
            )?;
        }
        if !self.flexible_total_distributed_rewards_principal.is_empty() {
            struct_ser.serialize_field(
                "flexibleTotalDistributedRewardsPrincipal",
                &self.flexible_total_distributed_rewards_principal,
            )?;
        }
        if !self.staked_total_principal.is_empty() {
            struct_ser.serialize_field("stakedTotalPrincipal", &self.staked_total_principal)?;
        }
        if self.staked_total_users != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "stakedTotalUsers",
                ToString::to_string(&self.staked_total_users).as_str(),
            )?;
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
            "flexible_total_principal",
            "flexibleTotalPrincipal",
            "flexible_total_users",
            "flexibleTotalUsers",
            "flexible_total_distributed_rewards_principal",
            "flexibleTotalDistributedRewardsPrincipal",
            "staked_total_principal",
            "stakedTotalPrincipal",
            "staked_total_users",
            "stakedTotalUsers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FlexibleTotalPrincipal,
            FlexibleTotalUsers,
            FlexibleTotalDistributedRewardsPrincipal,
            StakedTotalPrincipal,
            StakedTotalUsers,
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
                            "flexibleTotalPrincipal" | "flexible_total_principal" => {
                                Ok(GeneratedField::FlexibleTotalPrincipal)
                            }
                            "flexibleTotalUsers" | "flexible_total_users" => {
                                Ok(GeneratedField::FlexibleTotalUsers)
                            }
                            "flexibleTotalDistributedRewardsPrincipal"
                            | "flexible_total_distributed_rewards_principal" => {
                                Ok(GeneratedField::FlexibleTotalDistributedRewardsPrincipal)
                            }
                            "stakedTotalPrincipal" | "staked_total_principal" => {
                                Ok(GeneratedField::StakedTotalPrincipal)
                            }
                            "stakedTotalUsers" | "staked_total_users" => {
                                Ok(GeneratedField::StakedTotalUsers)
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
                formatter.write_str("struct noble.dollar.vaults.v1.Stats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Stats, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut flexible_total_principal__ = None;
                let mut flexible_total_users__ = None;
                let mut flexible_total_distributed_rewards_principal__ = None;
                let mut staked_total_principal__ = None;
                let mut staked_total_users__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FlexibleTotalPrincipal => {
                            if flexible_total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalPrincipal",
                                ));
                            }
                            flexible_total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FlexibleTotalUsers => {
                            if flexible_total_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalUsers",
                                ));
                            }
                            flexible_total_users__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FlexibleTotalDistributedRewardsPrincipal => {
                            if flexible_total_distributed_rewards_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "flexibleTotalDistributedRewardsPrincipal",
                                ));
                            }
                            flexible_total_distributed_rewards_principal__ =
                                Some(map_.next_value()?);
                        }
                        GeneratedField::StakedTotalPrincipal => {
                            if staked_total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "stakedTotalPrincipal",
                                ));
                            }
                            staked_total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StakedTotalUsers => {
                            if staked_total_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stakedTotalUsers"));
                            }
                            staked_total_users__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Stats {
                    flexible_total_principal: flexible_total_principal__.unwrap_or_default(),
                    flexible_total_users: flexible_total_users__.unwrap_or_default(),
                    flexible_total_distributed_rewards_principal:
                        flexible_total_distributed_rewards_principal__.unwrap_or_default(),
                    staked_total_principal: staked_total_principal__.unwrap_or_default(),
                    staked_total_users: staked_total_users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.vaults.v1.Stats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Staked => "STAKED",
            Self::Flexible => "FLEXIBLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VaultType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["UNSPECIFIED", "STAKED", "FLEXIBLE"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(VaultType::Unspecified),
                    "STAKED" => Ok(VaultType::Staked),
                    "FLEXIBLE" => Ok(VaultType::Flexible),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
