// @generated
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.portal.is_some() {
            len += 1;
        }
        if self.vaults.is_some() {
            len += 1;
        }
        if self.paused {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        if self.stats.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.GenesisState", len)?;
        if let Some(v) = self.portal.as_ref() {
            struct_ser.serialize_field("portal", v)?;
        }
        if let Some(v) = self.vaults.as_ref() {
            struct_ser.serialize_field("vaults", v)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
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
        const FIELDS: &[&str] = &["portal", "vaults", "paused", "index", "principal", "stats"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Portal,
            Vaults,
            Paused,
            Index,
            Principal,
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
                            "portal" => Ok(GeneratedField::Portal),
                            "vaults" => Ok(GeneratedField::Vaults),
                            "paused" => Ok(GeneratedField::Paused),
                            "index" => Ok(GeneratedField::Index),
                            "principal" => Ok(GeneratedField::Principal),
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
                formatter.write_str("struct noble.dollar.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut portal__ = None;
                let mut vaults__ = None;
                let mut paused__ = None;
                let mut index__ = None;
                let mut principal__ = None;
                let mut stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Portal => {
                            if portal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portal"));
                            }
                            portal__ = map_.next_value()?;
                        }
                        GeneratedField::Vaults => {
                            if vaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaults"));
                            }
                            vaults__ = map_.next_value()?;
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
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
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ =
                                Some(map_.next_value::<std::collections::HashMap<_, _>>()?);
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
                    portal: portal__,
                    vaults: vaults__,
                    paused: paused__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                    stats: stats__,
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IndexUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.old_index != 0 {
            len += 1;
        }
        if self.new_index != 0 {
            len += 1;
        }
        if !self.total_principal.is_empty() {
            len += 1;
        }
        if !self.yield_accrued.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.IndexUpdated", len)?;
        if self.old_index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("oldIndex", ToString::to_string(&self.old_index).as_str())?;
        }
        if self.new_index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("newIndex", ToString::to_string(&self.new_index).as_str())?;
        }
        if !self.total_principal.is_empty() {
            struct_ser.serialize_field("totalPrincipal", &self.total_principal)?;
        }
        if !self.yield_accrued.is_empty() {
            struct_ser.serialize_field("yieldAccrued", &self.yield_accrued)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IndexUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "old_index",
            "oldIndex",
            "new_index",
            "newIndex",
            "total_principal",
            "totalPrincipal",
            "yield_accrued",
            "yieldAccrued",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OldIndex,
            NewIndex,
            TotalPrincipal,
            YieldAccrued,
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
                            "oldIndex" | "old_index" => Ok(GeneratedField::OldIndex),
                            "newIndex" | "new_index" => Ok(GeneratedField::NewIndex),
                            "totalPrincipal" | "total_principal" => {
                                Ok(GeneratedField::TotalPrincipal)
                            }
                            "yieldAccrued" | "yield_accrued" => Ok(GeneratedField::YieldAccrued),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IndexUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.IndexUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IndexUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut old_index__ = None;
                let mut new_index__ = None;
                let mut total_principal__ = None;
                let mut yield_accrued__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OldIndex => {
                            if old_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldIndex"));
                            }
                            old_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NewIndex => {
                            if new_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newIndex"));
                            }
                            new_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalPrincipal => {
                            if total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPrincipal"));
                            }
                            total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::YieldAccrued => {
                            if yield_accrued__.is_some() {
                                return Err(serde::de::Error::duplicate_field("yieldAccrued"));
                            }
                            yield_accrued__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IndexUpdated {
                    old_index: old_index__.unwrap_or_default(),
                    new_index: new_index__.unwrap_or_default(),
                    total_principal: total_principal__.unwrap_or_default(),
                    yield_accrued: yield_accrued__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.IndexUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgClaimYield {
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
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.MsgClaimYield", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClaimYield {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClaimYield;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.MsgClaimYield")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgClaimYield, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgClaimYield {
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.MsgClaimYield", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgClaimYieldResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("noble.dollar.v1.MsgClaimYieldResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClaimYieldResponse {
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
            type Value = MsgClaimYieldResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.MsgClaimYieldResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgClaimYieldResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgClaimYieldResponse {})
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.v1.MsgClaimYieldResponse",
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
            serializer.serialize_struct("noble.dollar.v1.MsgSetPausedState", len)?;
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
                formatter.write_str("struct noble.dollar.v1.MsgSetPausedState")
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
            "noble.dollar.v1.MsgSetPausedState",
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
            serializer.serialize_struct("noble.dollar.v1.MsgSetPausedStateResponse", len)?;
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
                formatter.write_str("struct noble.dollar.v1.MsgSetPausedStateResponse")
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
            "noble.dollar.v1.MsgSetPausedStateResponse",
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
        let struct_ser = serializer.serialize_struct("noble.dollar.v1.Paused", len)?;
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
                formatter.write_str("struct noble.dollar.v1.Paused")
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
        deserializer.deserialize_struct("noble.dollar.v1.Paused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryIndex {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("noble.dollar.v1.QueryIndex", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIndex {
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
            type Value = QueryIndex;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryIndex")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryIndex, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryIndex {})
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.QueryIndex", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryIndexResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.index.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.v1.QueryIndexResponse", len)?;
        if !self.index.is_empty() {
            struct_ser.serialize_field("index", &self.index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIndexResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["index"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryIndexResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryIndexResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryIndexResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryIndexResponse {
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.v1.QueryIndexResponse",
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
        let struct_ser = serializer.serialize_struct("noble.dollar.v1.QueryPaused", len)?;
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
                formatter.write_str("struct noble.dollar.v1.QueryPaused")
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
        deserializer.deserialize_struct("noble.dollar.v1.QueryPaused", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("noble.dollar.v1.QueryPausedResponse", len)?;
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
                formatter.write_str("struct noble.dollar.v1.QueryPausedResponse")
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
            "noble.dollar.v1.QueryPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPrincipal {
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
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.QueryPrincipal", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPrincipal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["account"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPrincipal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryPrincipal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPrincipal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPrincipal {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.QueryPrincipal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryPrincipalResponse {
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
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.v1.QueryPrincipalResponse", len)?;
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPrincipalResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["principal"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryPrincipalResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryPrincipalResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPrincipalResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut principal__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPrincipalResponse {
                    principal: principal__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.v1.QueryPrincipalResponse",
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
        let struct_ser = serializer.serialize_struct("noble.dollar.v1.QueryStats", len)?;
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
                formatter.write_str("struct noble.dollar.v1.QueryStats")
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
        deserializer.deserialize_struct("noble.dollar.v1.QueryStats", FIELDS, GeneratedVisitor)
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
        if self.total_holders != 0 {
            len += 1;
        }
        if !self.total_principal.is_empty() {
            len += 1;
        }
        if !self.total_yield_accrued.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.v1.QueryStatsResponse", len)?;
        if self.total_holders != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "totalHolders",
                ToString::to_string(&self.total_holders).as_str(),
            )?;
        }
        if !self.total_principal.is_empty() {
            struct_ser.serialize_field("totalPrincipal", &self.total_principal)?;
        }
        if !self.total_yield_accrued.is_empty() {
            struct_ser.serialize_field("totalYieldAccrued", &self.total_yield_accrued)?;
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
            "total_holders",
            "totalHolders",
            "total_principal",
            "totalPrincipal",
            "total_yield_accrued",
            "totalYieldAccrued",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalHolders,
            TotalPrincipal,
            TotalYieldAccrued,
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
                            "totalHolders" | "total_holders" => Ok(GeneratedField::TotalHolders),
                            "totalPrincipal" | "total_principal" => {
                                Ok(GeneratedField::TotalPrincipal)
                            }
                            "totalYieldAccrued" | "total_yield_accrued" => {
                                Ok(GeneratedField::TotalYieldAccrued)
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
                formatter.write_str("struct noble.dollar.v1.QueryStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStatsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_holders__ = None;
                let mut total_principal__ = None;
                let mut total_yield_accrued__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalHolders => {
                            if total_holders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalHolders"));
                            }
                            total_holders__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalPrincipal => {
                            if total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPrincipal"));
                            }
                            total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalYieldAccrued => {
                            if total_yield_accrued__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalYieldAccrued"));
                            }
                            total_yield_accrued__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStatsResponse {
                    total_holders: total_holders__.unwrap_or_default(),
                    total_principal: total_principal__.unwrap_or_default(),
                    total_yield_accrued: total_yield_accrued__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.v1.QueryStatsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryYield {
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
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.QueryYield", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryYield {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["account"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryYield;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryYield")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryYield, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryYield {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.QueryYield", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryYieldResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.claimable_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("noble.dollar.v1.QueryYieldResponse", len)?;
        if !self.claimable_amount.is_empty() {
            struct_ser.serialize_field("claimableAmount", &self.claimable_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryYieldResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["claimable_amount", "claimableAmount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClaimableAmount,
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
                            "claimableAmount" | "claimable_amount" => {
                                Ok(GeneratedField::ClaimableAmount)
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
            type Value = QueryYieldResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.QueryYieldResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryYieldResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut claimable_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClaimableAmount => {
                            if claimable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimableAmount"));
                            }
                            claimable_amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryYieldResponse {
                    claimable_amount: claimable_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "noble.dollar.v1.QueryYieldResponse",
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
        if self.total_holders != 0 {
            len += 1;
        }
        if !self.total_principal.is_empty() {
            len += 1;
        }
        if !self.total_yield_accrued.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.Stats", len)?;
        if self.total_holders != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "totalHolders",
                ToString::to_string(&self.total_holders).as_str(),
            )?;
        }
        if !self.total_principal.is_empty() {
            struct_ser.serialize_field("totalPrincipal", &self.total_principal)?;
        }
        if !self.total_yield_accrued.is_empty() {
            struct_ser.serialize_field("totalYieldAccrued", &self.total_yield_accrued)?;
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
            "total_holders",
            "totalHolders",
            "total_principal",
            "totalPrincipal",
            "total_yield_accrued",
            "totalYieldAccrued",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalHolders,
            TotalPrincipal,
            TotalYieldAccrued,
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
                            "totalHolders" | "total_holders" => Ok(GeneratedField::TotalHolders),
                            "totalPrincipal" | "total_principal" => {
                                Ok(GeneratedField::TotalPrincipal)
                            }
                            "totalYieldAccrued" | "total_yield_accrued" => {
                                Ok(GeneratedField::TotalYieldAccrued)
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
                formatter.write_str("struct noble.dollar.v1.Stats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Stats, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_holders__ = None;
                let mut total_principal__ = None;
                let mut total_yield_accrued__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalHolders => {
                            if total_holders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalHolders"));
                            }
                            total_holders__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalPrincipal => {
                            if total_principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPrincipal"));
                            }
                            total_principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalYieldAccrued => {
                            if total_yield_accrued__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalYieldAccrued"));
                            }
                            total_yield_accrued__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Stats {
                    total_holders: total_holders__.unwrap_or_default(),
                    total_principal: total_principal__.unwrap_or_default(),
                    total_yield_accrued: total_yield_accrued__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.Stats", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("noble.dollar.v1.Unpaused", len)?;
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
                formatter.write_str("struct noble.dollar.v1.Unpaused")
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
        deserializer.deserialize_struct("noble.dollar.v1.Unpaused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for YieldClaimed {
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
        let mut struct_ser = serializer.serialize_struct("noble.dollar.v1.YieldClaimed", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for YieldClaimed {
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
            type Value = YieldClaimed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.v1.YieldClaimed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<YieldClaimed, V::Error>
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
                Ok(YieldClaimed {
                    account: account__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.v1.YieldClaimed", FIELDS, GeneratedVisitor)
    }
}
