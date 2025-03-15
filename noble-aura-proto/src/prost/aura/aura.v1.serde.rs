// @generated
impl serde::Serialize for BlockedChannelAdded {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.BlockedChannelAdded", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockedChannelAdded {
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
            type Value = BlockedChannelAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.BlockedChannelAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockedChannelAdded, V::Error>
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
                Ok(BlockedChannelAdded {
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.BlockedChannelAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockedChannelRemoved {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.BlockedChannelRemoved", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockedChannelRemoved {
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
            type Value = BlockedChannelRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.BlockedChannelRemoved")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BlockedChannelRemoved, V::Error>
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
                Ok(BlockedChannelRemoved {
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.BlockedChannelRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Burner {
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
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.Burner", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Burner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Allowance,
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
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Burner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.Burner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Burner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Burner {
                    address: address__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.Burner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BurnerAdded {
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
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.BurnerAdded", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurnerAdded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Allowance,
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
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BurnerAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.BurnerAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BurnerAdded, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BurnerAdded {
                    address: address__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.BurnerAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BurnerRemoved {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.BurnerRemoved", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurnerRemoved {
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
            type Value = BurnerRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.BurnerRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BurnerRemoved, V::Error>
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
                Ok(BurnerRemoved {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.BurnerRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BurnerUpdated {
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
        if !self.previous_allowance.is_empty() {
            len += 1;
        }
        if !self.new_allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.BurnerUpdated", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.previous_allowance.is_empty() {
            struct_ser.serialize_field("previousAllowance", &self.previous_allowance)?;
        }
        if !self.new_allowance.is_empty() {
            struct_ser.serialize_field("newAllowance", &self.new_allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BurnerUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "previous_allowance",
            "previousAllowance",
            "new_allowance",
            "newAllowance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            PreviousAllowance,
            NewAllowance,
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
                            "previousAllowance" | "previous_allowance" => {
                                Ok(GeneratedField::PreviousAllowance)
                            }
                            "newAllowance" | "new_allowance" => Ok(GeneratedField::NewAllowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BurnerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.BurnerUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BurnerUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut previous_allowance__ = None;
                let mut new_allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreviousAllowance => {
                            if previous_allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousAllowance"));
                            }
                            previous_allowance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAllowance => {
                            if new_allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAllowance"));
                            }
                            new_allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BurnerUpdated {
                    address: address__.unwrap_or_default(),
                    previous_allowance: previous_allowance__.unwrap_or_default(),
                    new_allowance: new_allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.BurnerUpdated", FIELDS, GeneratedVisitor)
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
        if self.blocklist_state.is_some() {
            len += 1;
        }
        if self.paused {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.pending_owner.is_empty() {
            len += 1;
        }
        if !self.burners.is_empty() {
            len += 1;
        }
        if !self.minters.is_empty() {
            len += 1;
        }
        if !self.pausers.is_empty() {
            len += 1;
        }
        if !self.blocked_channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.GenesisState", len)?;
        if let Some(v) = self.blocklist_state.as_ref() {
            struct_ser.serialize_field("blocklistState", v)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.pending_owner.is_empty() {
            struct_ser.serialize_field("pendingOwner", &self.pending_owner)?;
        }
        if !self.burners.is_empty() {
            struct_ser.serialize_field("burners", &self.burners)?;
        }
        if !self.minters.is_empty() {
            struct_ser.serialize_field("minters", &self.minters)?;
        }
        if !self.pausers.is_empty() {
            struct_ser.serialize_field("pausers", &self.pausers)?;
        }
        if !self.blocked_channels.is_empty() {
            struct_ser.serialize_field("blockedChannels", &self.blocked_channels)?;
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
            "blocklist_state",
            "blocklistState",
            "paused",
            "owner",
            "pending_owner",
            "pendingOwner",
            "burners",
            "minters",
            "pausers",
            "blocked_channels",
            "blockedChannels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlocklistState,
            Paused,
            Owner,
            PendingOwner,
            Burners,
            Minters,
            Pausers,
            BlockedChannels,
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
                            "blocklistState" | "blocklist_state" => {
                                Ok(GeneratedField::BlocklistState)
                            }
                            "paused" => Ok(GeneratedField::Paused),
                            "owner" => Ok(GeneratedField::Owner),
                            "pendingOwner" | "pending_owner" => Ok(GeneratedField::PendingOwner),
                            "burners" => Ok(GeneratedField::Burners),
                            "minters" => Ok(GeneratedField::Minters),
                            "pausers" => Ok(GeneratedField::Pausers),
                            "blockedChannels" | "blocked_channels" => {
                                Ok(GeneratedField::BlockedChannels)
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
                formatter.write_str("struct aura.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocklist_state__ = None;
                let mut paused__ = None;
                let mut owner__ = None;
                let mut pending_owner__ = None;
                let mut burners__ = None;
                let mut minters__ = None;
                let mut pausers__ = None;
                let mut blocked_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlocklistState => {
                            if blocklist_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocklistState"));
                            }
                            blocklist_state__ = map_.next_value()?;
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PendingOwner => {
                            if pending_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingOwner"));
                            }
                            pending_owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Burners => {
                            if burners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burners"));
                            }
                            burners__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Minters => {
                            if minters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minters"));
                            }
                            minters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pausers => {
                            if pausers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pausers"));
                            }
                            pausers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockedChannels => {
                            if blocked_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockedChannels"));
                            }
                            blocked_channels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    blocklist_state: blocklist_state__,
                    paused: paused__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    pending_owner: pending_owner__.unwrap_or_default(),
                    burners: burners__.unwrap_or_default(),
                    minters: minters__.unwrap_or_default(),
                    pausers: pausers__.unwrap_or_default(),
                    blocked_channels: blocked_channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Minter {
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
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.Minter", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Minter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Allowance,
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
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Minter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.Minter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Minter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Minter {
                    address: address__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.Minter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinterAdded {
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
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MinterAdded", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinterAdded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Allowance,
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
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MinterAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MinterAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MinterAdded, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MinterAdded {
                    address: address__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MinterAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinterRemoved {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.MinterRemoved", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinterRemoved {
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
            type Value = MinterRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MinterRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MinterRemoved, V::Error>
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
                Ok(MinterRemoved {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MinterRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinterUpdated {
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
        if !self.previous_allowance.is_empty() {
            len += 1;
        }
        if !self.new_allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MinterUpdated", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.previous_allowance.is_empty() {
            struct_ser.serialize_field("previousAllowance", &self.previous_allowance)?;
        }
        if !self.new_allowance.is_empty() {
            struct_ser.serialize_field("newAllowance", &self.new_allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinterUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "previous_allowance",
            "previousAllowance",
            "new_allowance",
            "newAllowance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            PreviousAllowance,
            NewAllowance,
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
                            "previousAllowance" | "previous_allowance" => {
                                Ok(GeneratedField::PreviousAllowance)
                            }
                            "newAllowance" | "new_allowance" => Ok(GeneratedField::NewAllowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MinterUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MinterUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MinterUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut previous_allowance__ = None;
                let mut new_allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreviousAllowance => {
                            if previous_allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousAllowance"));
                            }
                            previous_allowance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAllowance => {
                            if new_allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAllowance"));
                            }
                            new_allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MinterUpdated {
                    address: address__.unwrap_or_default(),
                    previous_allowance: previous_allowance__.unwrap_or_default(),
                    new_allowance: new_allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MinterUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAcceptOwnership {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgAcceptOwnership", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptOwnership {
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
            type Value = MsgAcceptOwnership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAcceptOwnership")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAcceptOwnership, V::Error>
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
                Ok(MsgAcceptOwnership {
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAcceptOwnership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAcceptOwnershipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgAcceptOwnershipResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptOwnershipResponse {
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
            type Value = MsgAcceptOwnershipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAcceptOwnershipResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAcceptOwnershipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAcceptOwnershipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.MsgAcceptOwnershipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddBlockedChannel {
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
        if !self.channel.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgAddBlockedChannel", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddBlockedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "channel"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
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
                            "signer" => Ok(GeneratedField::Signer),
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
            type Value = MsgAddBlockedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddBlockedChannel")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddBlockedChannel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddBlockedChannel {
                    signer: signer__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddBlockedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddBlockedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("aura.v1.MsgAddBlockedChannelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddBlockedChannelResponse {
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
            type Value = MsgAddBlockedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddBlockedChannelResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddBlockedChannelResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddBlockedChannelResponse {})
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.MsgAddBlockedChannelResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddBurner {
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
        if !self.burner.is_empty() {
            len += 1;
        }
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgAddBurner", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.burner.is_empty() {
            struct_ser.serialize_field("burner", &self.burner)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddBurner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "burner", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Burner,
            Allowance,
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
                            "burner" => Ok(GeneratedField::Burner),
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddBurner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddBurner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAddBurner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut burner__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Burner => {
                            if burner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burner"));
                            }
                            burner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddBurner {
                    signer: signer__.unwrap_or_default(),
                    burner: burner__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddBurner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddBurnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgAddBurnerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddBurnerResponse {
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
            type Value = MsgAddBurnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddBurnerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddBurnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddBurnerResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddBurnerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddMinter {
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
        if !self.minter.is_empty() {
            len += 1;
        }
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgAddMinter", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.minter.is_empty() {
            struct_ser.serialize_field("minter", &self.minter)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddMinter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "minter", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Minter,
            Allowance,
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
                            "minter" => Ok(GeneratedField::Minter),
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddMinter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddMinter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAddMinter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut minter__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Minter => {
                            if minter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minter"));
                            }
                            minter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddMinter {
                    signer: signer__.unwrap_or_default(),
                    minter: minter__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddMinter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddMinterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgAddMinterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddMinterResponse {
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
            type Value = MsgAddMinterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddMinterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddMinterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddMinterResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddMinterResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddPauser {
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
        if !self.pauser.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgAddPauser", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.pauser.is_empty() {
            struct_ser.serialize_field("pauser", &self.pauser)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddPauser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "pauser"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Pauser,
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
                            "pauser" => Ok(GeneratedField::Pauser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddPauser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddPauser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAddPauser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut pauser__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pauser => {
                            if pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauser"));
                            }
                            pauser__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAddPauser {
                    signer: signer__.unwrap_or_default(),
                    pauser: pauser__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddPauser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddPauserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgAddPauserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddPauserResponse {
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
            type Value = MsgAddPauserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgAddPauserResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAddPauserResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddPauserResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgAddPauserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBurn {
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
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgBurn", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBurn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "from", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
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
                            "signer" => Ok(GeneratedField::Signer),
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
            type Value = MsgBurn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgBurn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgBurn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut from__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
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
                    }
                }
                Ok(MsgBurn {
                    signer: signer__.unwrap_or_default(),
                    from: from__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgBurn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBurnResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgBurnResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBurnResponse {
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
            type Value = MsgBurnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgBurnResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgBurnResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBurnResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgBurnResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgMint {
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
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgMint", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "to", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            To,
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
                            "to" => Ok(GeneratedField::To),
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
            type Value = MsgMint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgMint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgMint, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut to__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgMint {
                    signer: signer__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgMint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgMintResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgMintResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMintResponse {
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
            type Value = MsgMintResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgMintResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgMintResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgMintResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgMintResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgPause", len)?;
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
                formatter.write_str("struct aura.v1.MsgPause")
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
        deserializer.deserialize_struct("aura.v1.MsgPause", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("aura.v1.MsgPauseResponse", len)?;
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
                formatter.write_str("struct aura.v1.MsgPauseResponse")
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
        deserializer.deserialize_struct("aura.v1.MsgPauseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveBlockedChannel {
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
        if !self.channel.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgRemoveBlockedChannel", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveBlockedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "channel"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
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
                            "signer" => Ok(GeneratedField::Signer),
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
            type Value = MsgRemoveBlockedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveBlockedChannel")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveBlockedChannel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveBlockedChannel {
                    signer: signer__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemoveBlockedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveBlockedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("aura.v1.MsgRemoveBlockedChannelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveBlockedChannelResponse {
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
            type Value = MsgRemoveBlockedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveBlockedChannelResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveBlockedChannelResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveBlockedChannelResponse {})
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.MsgRemoveBlockedChannelResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveBurner {
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
        if !self.burner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgRemoveBurner", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.burner.is_empty() {
            struct_ser.serialize_field("burner", &self.burner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveBurner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "burner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Burner,
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
                            "burner" => Ok(GeneratedField::Burner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveBurner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveBurner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRemoveBurner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut burner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Burner => {
                            if burner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burner"));
                            }
                            burner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveBurner {
                    signer: signer__.unwrap_or_default(),
                    burner: burner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemoveBurner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveBurnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgRemoveBurnerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveBurnerResponse {
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
            type Value = MsgRemoveBurnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveBurnerResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveBurnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveBurnerResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemoveBurnerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveMinter {
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
        if !self.minter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgRemoveMinter", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.minter.is_empty() {
            struct_ser.serialize_field("minter", &self.minter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveMinter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "minter"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Minter,
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
                            "minter" => Ok(GeneratedField::Minter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveMinter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveMinter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRemoveMinter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut minter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Minter => {
                            if minter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minter"));
                            }
                            minter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveMinter {
                    signer: signer__.unwrap_or_default(),
                    minter: minter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemoveMinter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveMinterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgRemoveMinterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveMinterResponse {
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
            type Value = MsgRemoveMinterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemoveMinterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemoveMinterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveMinterResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemoveMinterResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemovePauser {
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
        if !self.pauser.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgRemovePauser", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.pauser.is_empty() {
            struct_ser.serialize_field("pauser", &self.pauser)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemovePauser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "pauser"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Pauser,
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
                            "pauser" => Ok(GeneratedField::Pauser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemovePauser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemovePauser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRemovePauser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut pauser__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pauser => {
                            if pauser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauser"));
                            }
                            pauser__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemovePauser {
                    signer: signer__.unwrap_or_default(),
                    pauser: pauser__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemovePauser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemovePauserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.MsgRemovePauserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemovePauserResponse {
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
            type Value = MsgRemovePauserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgRemovePauserResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRemovePauserResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemovePauserResponse {})
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgRemovePauserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSetBurnerAllowance {
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
        if !self.burner.is_empty() {
            len += 1;
        }
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgSetBurnerAllowance", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.burner.is_empty() {
            struct_ser.serialize_field("burner", &self.burner)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetBurnerAllowance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "burner", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Burner,
            Allowance,
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
                            "burner" => Ok(GeneratedField::Burner),
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetBurnerAllowance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgSetBurnerAllowance")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetBurnerAllowance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut burner__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Burner => {
                            if burner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burner"));
                            }
                            burner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetBurnerAllowance {
                    signer: signer__.unwrap_or_default(),
                    burner: burner__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgSetBurnerAllowance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSetBurnerAllowanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("aura.v1.MsgSetBurnerAllowanceResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetBurnerAllowanceResponse {
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
            type Value = MsgSetBurnerAllowanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgSetBurnerAllowanceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetBurnerAllowanceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetBurnerAllowanceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.MsgSetBurnerAllowanceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetMinterAllowance {
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
        if !self.minter.is_empty() {
            len += 1;
        }
        if !self.allowance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgSetMinterAllowance", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.minter.is_empty() {
            struct_ser.serialize_field("minter", &self.minter)?;
        }
        if !self.allowance.is_empty() {
            struct_ser.serialize_field("allowance", &self.allowance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetMinterAllowance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "minter", "allowance"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Minter,
            Allowance,
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
                            "minter" => Ok(GeneratedField::Minter),
                            "allowance" => Ok(GeneratedField::Allowance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetMinterAllowance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgSetMinterAllowance")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetMinterAllowance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut minter__ = None;
                let mut allowance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Minter => {
                            if minter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minter"));
                            }
                            minter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Allowance => {
                            if allowance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowance"));
                            }
                            allowance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetMinterAllowance {
                    signer: signer__.unwrap_or_default(),
                    minter: minter__.unwrap_or_default(),
                    allowance: allowance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.MsgSetMinterAllowance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSetMinterAllowanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("aura.v1.MsgSetMinterAllowanceResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetMinterAllowanceResponse {
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
            type Value = MsgSetMinterAllowanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.MsgSetMinterAllowanceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetMinterAllowanceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetMinterAllowanceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.MsgSetMinterAllowanceResponse",
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgTransferOwnership", len)?;
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
                formatter.write_str("struct aura.v1.MsgTransferOwnership")
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
        deserializer.deserialize_struct("aura.v1.MsgTransferOwnership", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("aura.v1.MsgTransferOwnershipResponse", len)?;
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
                formatter.write_str("struct aura.v1.MsgTransferOwnershipResponse")
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
            "aura.v1.MsgTransferOwnershipResponse",
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.MsgUnpause", len)?;
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
                formatter.write_str("struct aura.v1.MsgUnpause")
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
        deserializer.deserialize_struct("aura.v1.MsgUnpause", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("aura.v1.MsgUnpauseResponse", len)?;
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
                formatter.write_str("struct aura.v1.MsgUnpauseResponse")
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
        deserializer.deserialize_struct("aura.v1.MsgUnpauseResponse", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("aura.v1.OwnershipTransferStarted", len)?;
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
                formatter.write_str("struct aura.v1.OwnershipTransferStarted")
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
            "aura.v1.OwnershipTransferStarted",
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.OwnershipTransferred", len)?;
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
                formatter.write_str("struct aura.v1.OwnershipTransferred")
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
        deserializer.deserialize_struct("aura.v1.OwnershipTransferred", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.Paused", len)?;
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
                formatter.write_str("struct aura.v1.Paused")
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
        deserializer.deserialize_struct("aura.v1.Paused", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PauserAdded {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.PauserAdded", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PauserAdded {
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
            type Value = PauserAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.PauserAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PauserAdded, V::Error>
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
                Ok(PauserAdded {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.PauserAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PauserRemoved {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.PauserRemoved", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PauserRemoved {
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
            type Value = PauserRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.PauserRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PauserRemoved, V::Error>
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
                Ok(PauserRemoved {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.PauserRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBlockedChannels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.QueryBlockedChannels", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockedChannels {
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
            type Value = QueryBlockedChannels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryBlockedChannels")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBlockedChannels, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBlockedChannels {})
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryBlockedChannels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBlockedChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocked_channels.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("aura.v1.QueryBlockedChannelsResponse", len)?;
        if !self.blocked_channels.is_empty() {
            struct_ser.serialize_field("blockedChannels", &self.blocked_channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockedChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["blocked_channels", "blockedChannels"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockedChannels,
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
                            "blockedChannels" | "blocked_channels" => {
                                Ok(GeneratedField::BlockedChannels)
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
            type Value = QueryBlockedChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryBlockedChannelsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBlockedChannelsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocked_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockedChannels => {
                            if blocked_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockedChannels"));
                            }
                            blocked_channels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBlockedChannelsResponse {
                    blocked_channels: blocked_channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "aura.v1.QueryBlockedChannelsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBurners {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.QueryBurners", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBurners {
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
            type Value = QueryBurners;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryBurners")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBurners, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBurners {})
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryBurners", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBurnersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.burners.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryBurnersResponse", len)?;
        if !self.burners.is_empty() {
            struct_ser.serialize_field("burners", &self.burners)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBurnersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["burners"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Burners,
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
                            "burners" => Ok(GeneratedField::Burners),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBurnersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryBurnersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBurnersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut burners__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Burners => {
                            if burners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burners"));
                            }
                            burners__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBurnersResponse {
                    burners: burners__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryBurnersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.QueryDenom", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDenom {
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
            type Value = QueryDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryDenom, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryDenom {})
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryDenomResponse {
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryDenomResponse", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDenomResponse {
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
            type Value = QueryDenomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryDenomResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryDenomResponse, V::Error>
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
                Ok(QueryDenomResponse {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryDenomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryMinters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.QueryMinters", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMinters {
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
            type Value = QueryMinters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryMinters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryMinters, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryMinters {})
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryMinters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryMintersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.minters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryMintersResponse", len)?;
        if !self.minters.is_empty() {
            struct_ser.serialize_field("minters", &self.minters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMintersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["minters"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Minters,
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
                            "minters" => Ok(GeneratedField::Minters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryMintersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryMintersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryMintersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut minters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Minters => {
                            if minters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minters"));
                            }
                            minters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryMintersResponse {
                    minters: minters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryMintersResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("aura.v1.QueryOwner", len)?;
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
                formatter.write_str("struct aura.v1.QueryOwner")
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
        deserializer.deserialize_struct("aura.v1.QueryOwner", FIELDS, GeneratedVisitor)
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
        if !self.pending_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryOwnerResponse", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.pending_owner.is_empty() {
            struct_ser.serialize_field("pendingOwner", &self.pending_owner)?;
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
        const FIELDS: &[&str] = &["owner", "pending_owner", "pendingOwner"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            PendingOwner,
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
                            "pendingOwner" | "pending_owner" => Ok(GeneratedField::PendingOwner),
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
                formatter.write_str("struct aura.v1.QueryOwnerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryOwnerResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut pending_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PendingOwner => {
                            if pending_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingOwner"));
                            }
                            pending_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOwnerResponse {
                    owner: owner__.unwrap_or_default(),
                    pending_owner: pending_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryOwnerResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("aura.v1.QueryPaused", len)?;
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
                formatter.write_str("struct aura.v1.QueryPaused")
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
        deserializer.deserialize_struct("aura.v1.QueryPaused", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryPausedResponse", len)?;
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
                formatter.write_str("struct aura.v1.QueryPausedResponse")
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
        deserializer.deserialize_struct("aura.v1.QueryPausedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryPausers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aura.v1.QueryPausers", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPausers {
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
            type Value = QueryPausers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryPausers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryPausers, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPausers {})
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryPausers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryPausersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pausers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aura.v1.QueryPausersResponse", len)?;
        if !self.pausers.is_empty() {
            struct_ser.serialize_field("pausers", &self.pausers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPausersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pausers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pausers,
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
                            "pausers" => Ok(GeneratedField::Pausers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPausersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aura.v1.QueryPausersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPausersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pausers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pausers => {
                            if pausers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pausers"));
                            }
                            pausers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPausersResponse {
                    pausers: pausers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aura.v1.QueryPausersResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("aura.v1.Unpaused", len)?;
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
                formatter.write_str("struct aura.v1.Unpaused")
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
        deserializer.deserialize_struct("aura.v1.Unpaused", FIELDS, GeneratedVisitor)
    }
}
