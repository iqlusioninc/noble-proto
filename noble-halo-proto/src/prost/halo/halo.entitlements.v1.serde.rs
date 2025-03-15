// @generated
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
        if !self.public_capabilities.is_empty() {
            len += 1;
        }
        if !self.role_capabilities.is_empty() {
            len += 1;
        }
        if !self.user_roles.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.GenesisState", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        if !self.public_capabilities.is_empty() {
            struct_ser.serialize_field("publicCapabilities", &self.public_capabilities)?;
        }
        if !self.role_capabilities.is_empty() {
            struct_ser.serialize_field("roleCapabilities", &self.role_capabilities)?;
        }
        if !self.user_roles.is_empty() {
            struct_ser.serialize_field("userRoles", &self.user_roles)?;
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
            "public_capabilities",
            "publicCapabilities",
            "role_capabilities",
            "roleCapabilities",
            "user_roles",
            "userRoles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            Paused,
            PublicCapabilities,
            RoleCapabilities,
            UserRoles,
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
                            "publicCapabilities" | "public_capabilities" => {
                                Ok(GeneratedField::PublicCapabilities)
                            }
                            "roleCapabilities" | "role_capabilities" => {
                                Ok(GeneratedField::RoleCapabilities)
                            }
                            "userRoles" | "user_roles" => Ok(GeneratedField::UserRoles),
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
                formatter.write_str("struct halo.entitlements.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut paused__ = None;
                let mut public_capabilities__ = None;
                let mut role_capabilities__ = None;
                let mut user_roles__ = None;
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
                        GeneratedField::PublicCapabilities => {
                            if public_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "publicCapabilities",
                                ));
                            }
                            public_capabilities__ =
                                Some(map_.next_value::<std::collections::HashMap<_, _>>()?);
                        }
                        GeneratedField::RoleCapabilities => {
                            if role_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleCapabilities"));
                            }
                            role_capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserRoles => {
                            if user_roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userRoles"));
                            }
                            user_roles__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    owner: owner__.unwrap_or_default(),
                    paused: paused__.unwrap_or_default(),
                    public_capabilities: public_capabilities__.unwrap_or_default(),
                    role_capabilities: role_capabilities__.unwrap_or_default(),
                    user_roles: user_roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgPause {
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
        let mut struct_ser = serializer.serialize_struct("halo.entitlements.v1.MsgPause", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPause {
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
            type Value = MsgPause;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgPause")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPause, V::Error>
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
                Ok(MsgPause {
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.entitlements.v1.MsgPause", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgPauseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgPauseResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPauseResponse {
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
            type Value = MsgPauseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgPauseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPauseResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgPauseResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgPauseResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPublicCapability {
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
        if !self.method.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgSetPublicCapability", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPublicCapability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "method", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Method,
            Enabled,
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
                            "method" => Ok(GeneratedField::Method),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetPublicCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetPublicCapability")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetPublicCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut method__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetPublicCapability {
                    signer: signer__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetPublicCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetPublicCapabilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("halo.entitlements.v1.MsgSetPublicCapabilityResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetPublicCapabilityResponse {
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
            type Value = MsgSetPublicCapabilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetPublicCapabilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetPublicCapabilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetPublicCapabilityResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetPublicCapabilityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetRoleCapability {
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
        if self.role != 0 {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgSetRoleCapability", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetRoleCapability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "role", "method", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Role,
            Method,
            Enabled,
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
                            "role" => Ok(GeneratedField::Role),
                            "method" => Ok(GeneratedField::Method),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetRoleCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetRoleCapability")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetRoleCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut role__ = None;
                let mut method__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetRoleCapability {
                    signer: signer__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetRoleCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetRoleCapabilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("halo.entitlements.v1.MsgSetRoleCapabilityResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetRoleCapabilityResponse {
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
            type Value = MsgSetRoleCapabilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetRoleCapabilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetRoleCapabilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetRoleCapabilityResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetRoleCapabilityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetUserRole {
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
        if !self.user.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgSetUserRole", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetUserRole {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "user", "role", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            User,
            Role,
            Enabled,
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
                            "user" => Ok(GeneratedField::User),
                            "role" => Ok(GeneratedField::Role),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetUserRole;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetUserRole")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetUserRole, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut user__ = None;
                let mut role__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetUserRole {
                    signer: signer__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetUserRole",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetUserRoleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgSetUserRoleResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetUserRoleResponse {
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
            type Value = MsgSetUserRoleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgSetUserRoleResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetUserRoleResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetUserRoleResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgSetUserRoleResponse",
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
            serializer.serialize_struct("halo.entitlements.v1.MsgTransferOwnership", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.MsgTransferOwnership")
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
            "halo.entitlements.v1.MsgTransferOwnership",
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
            .serialize_struct("halo.entitlements.v1.MsgTransferOwnershipResponse", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.MsgTransferOwnershipResponse")
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
            "halo.entitlements.v1.MsgTransferOwnershipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnpause {
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
        let mut struct_ser = serializer.serialize_struct("halo.entitlements.v1.MsgUnpause", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpause {
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
            type Value = MsgUnpause;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgUnpause")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnpause, V::Error>
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
                Ok(MsgUnpause {
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.entitlements.v1.MsgUnpause", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUnpauseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.MsgUnpauseResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnpauseResponse {
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
            type Value = MsgUnpauseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.MsgUnpauseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnpauseResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnpauseResponse {})
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.MsgUnpauseResponse",
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
            serializer.serialize_struct("halo.entitlements.v1.OwnershipTransferred", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.OwnershipTransferred")
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
            "halo.entitlements.v1.OwnershipTransferred",
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
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("halo.entitlements.v1.Paused", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Paused {
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
            type Value = Paused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.Paused")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Paused, V::Error>
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
                Ok(Paused {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.entitlements.v1.Paused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicCapabilityUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.PublicCapabilityUpdated", len)?;
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicCapabilityUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["method", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Enabled,
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
                            "method" => Ok(GeneratedField::Method),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicCapabilityUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.PublicCapabilityUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PublicCapabilityUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PublicCapabilityUpdated {
                    method: method__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.PublicCapabilityUpdated",
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
        let struct_ser = serializer.serialize_struct("halo.entitlements.v1.QueryOwner", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.QueryOwner")
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
        deserializer.deserialize_struct("halo.entitlements.v1.QueryOwner", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("halo.entitlements.v1.QueryOwnerResponse", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.QueryOwnerResponse")
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
            "halo.entitlements.v1.QueryOwnerResponse",
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
        let struct_ser = serializer.serialize_struct("halo.entitlements.v1.QueryPaused", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.QueryPaused")
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
            "halo.entitlements.v1.QueryPaused",
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
            serializer.serialize_struct("halo.entitlements.v1.QueryPausedResponse", len)?;
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
                formatter.write_str("struct halo.entitlements.v1.QueryPausedResponse")
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
            "halo.entitlements.v1.QueryPausedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPublicCapability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.QueryPublicCapability", len)?;
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPublicCapability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["method"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
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
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPublicCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryPublicCapability")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPublicCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPublicCapability {
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryPublicCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryPublicCapabilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("halo.entitlements.v1.QueryPublicCapabilityResponse", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPublicCapabilityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPublicCapabilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryPublicCapabilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPublicCapabilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPublicCapabilityResponse {
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryPublicCapabilityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoleCapability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.QueryRoleCapability", len)?;
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoleCapability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["method"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
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
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRoleCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryRoleCapability")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRoleCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRoleCapability {
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryRoleCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRoleCapabilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.QueryRoleCapabilityResponse", len)?;
        if !self.roles.is_empty() {
            let v = self
                .roles
                .iter()
                .cloned()
                .map(|v| {
                    Role::try_from(v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("roles", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRoleCapabilityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["roles"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Roles,
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
                            "roles" => Ok(GeneratedField::Roles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRoleCapabilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryRoleCapabilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryRoleCapabilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut roles__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(
                                map_.next_value::<Vec<Role>>()?
                                    .into_iter()
                                    .map(|x| x as i32)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(QueryRoleCapabilityResponse {
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryRoleCapabilityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryUserCapability {
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
            serializer.serialize_struct("halo.entitlements.v1.QueryUserCapability", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUserCapability {
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
            type Value = QueryUserCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryUserCapability")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryUserCapability, V::Error>
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
                Ok(QueryUserCapability {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryUserCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryUserCapabilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.QueryUserCapabilityResponse", len)?;
        if !self.roles.is_empty() {
            let v = self
                .roles
                .iter()
                .cloned()
                .map(|v| {
                    Role::try_from(v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("roles", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUserCapabilityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["roles"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Roles,
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
                            "roles" => Ok(GeneratedField::Roles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUserCapabilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.QueryUserCapabilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryUserCapabilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut roles__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(
                                map_.next_value::<Vec<Role>>()?
                                    .into_iter()
                                    .map(|x| x as i32)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(QueryUserCapabilityResponse {
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.QueryUserCapabilityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Role {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ROLE_UNSPECIFIED",
            Self::DomesticFeeder => "ROLE_DOMESTIC_FEEDER",
            Self::InternationalFeeder => "ROLE_INTERNATIONAL_FEEDER",
            Self::DomesticSdyf => "ROLE_DOMESTIC_SDYF",
            Self::InternationalSdyf => "ROLE_INTERNATIONAL_SDYF",
            Self::FundAdmin => "ROLE_FUND_ADMIN",
            Self::LiquidityProvider => "ROLE_LIQUIDITY_PROVIDER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Role {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ROLE_UNSPECIFIED",
            "ROLE_DOMESTIC_FEEDER",
            "ROLE_INTERNATIONAL_FEEDER",
            "ROLE_DOMESTIC_SDYF",
            "ROLE_INTERNATIONAL_SDYF",
            "ROLE_FUND_ADMIN",
            "ROLE_LIQUIDITY_PROVIDER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Role;

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
                    "ROLE_UNSPECIFIED" => Ok(Role::Unspecified),
                    "ROLE_DOMESTIC_FEEDER" => Ok(Role::DomesticFeeder),
                    "ROLE_INTERNATIONAL_FEEDER" => Ok(Role::InternationalFeeder),
                    "ROLE_DOMESTIC_SDYF" => Ok(Role::DomesticSdyf),
                    "ROLE_INTERNATIONAL_SDYF" => Ok(Role::InternationalSdyf),
                    "ROLE_FUND_ADMIN" => Ok(Role::FundAdmin),
                    "ROLE_LIQUIDITY_PROVIDER" => Ok(Role::LiquidityProvider),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RoleCapability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.RoleCapability", len)?;
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleCapability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["method", "role", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Role,
            Enabled,
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
                            "method" => Ok(GeneratedField::Method),
                            "role" => Ok(GeneratedField::Role),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoleCapability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.RoleCapability")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoleCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                let mut role__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoleCapability {
                    method: method__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.RoleCapability",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RoleCapabilityUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.role != 0 {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.RoleCapabilityUpdated", len)?;
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleCapabilityUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["role", "method", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Role,
            Method,
            Enabled,
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
                            "role" => Ok(GeneratedField::Role),
                            "method" => Ok(GeneratedField::Method),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoleCapabilityUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.RoleCapabilityUpdated")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RoleCapabilityUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut role__ = None;
                let mut method__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoleCapabilityUpdated {
                    role: role__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.RoleCapabilityUpdated",
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
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("halo.entitlements.v1.Unpaused", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Unpaused {
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
            type Value = Unpaused;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.Unpaused")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Unpaused, V::Error>
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
                Ok(Unpaused {
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.entitlements.v1.Unpaused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserRole {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("halo.entitlements.v1.UserRole", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserRole {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "role", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Role,
            Enabled,
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
                            "user" => Ok(GeneratedField::User),
                            "role" => Ok(GeneratedField::Role),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserRole;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.UserRole")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserRole, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut role__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserRole {
                    user: user__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("halo.entitlements.v1.UserRole", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserRoleUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("halo.entitlements.v1.UserRoleUpdated", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserRoleUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "role", "enabled"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Role,
            Enabled,
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
                            "user" => Ok(GeneratedField::User),
                            "role" => Ok(GeneratedField::Role),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserRoleUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct halo.entitlements.v1.UserRoleUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserRoleUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut role__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserRoleUpdated {
                    user: user__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "halo.entitlements.v1.UserRoleUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
