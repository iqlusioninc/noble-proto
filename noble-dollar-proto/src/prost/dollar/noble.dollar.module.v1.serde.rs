// @generated
impl serde::Serialize for Module {
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
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.vaults_minimum_lock != 0 {
            len += 1;
        }
        if self.vaults_minimum_unlock != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("noble.dollar.module.v1.Module", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if self.vaults_minimum_lock != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "vaultsMinimumLock",
                ToString::to_string(&self.vaults_minimum_lock).as_str(),
            )?;
        }
        if self.vaults_minimum_unlock != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "vaultsMinimumUnlock",
                ToString::to_string(&self.vaults_minimum_unlock).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "authority",
            "vaults_minimum_lock",
            "vaultsMinimumLock",
            "vaults_minimum_unlock",
            "vaultsMinimumUnlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            Authority,
            VaultsMinimumLock,
            VaultsMinimumUnlock,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "vaultsMinimumLock" | "vaults_minimum_lock" => {
                                Ok(GeneratedField::VaultsMinimumLock)
                            }
                            "vaultsMinimumUnlock" | "vaults_minimum_unlock" => {
                                Ok(GeneratedField::VaultsMinimumUnlock)
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
            type Value = Module;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct noble.dollar.module.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Module, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut authority__ = None;
                let mut vaults_minimum_lock__ = None;
                let mut vaults_minimum_unlock__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VaultsMinimumLock => {
                            if vaults_minimum_lock__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultsMinimumLock"));
                            }
                            vaults_minimum_lock__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::VaultsMinimumUnlock => {
                            if vaults_minimum_unlock__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "vaultsMinimumUnlock",
                                ));
                            }
                            vaults_minimum_unlock__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Module {
                    denom: denom__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    vaults_minimum_lock: vaults_minimum_lock__.unwrap_or_default(),
                    vaults_minimum_unlock: vaults_minimum_unlock__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("noble.dollar.module.v1.Module", FIELDS, GeneratedVisitor)
    }
}
