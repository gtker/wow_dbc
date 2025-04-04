/// DBCs from the English version of the game will only have English version strings, while other localizations will have other languages.
///
/// You are most likely interested in, [`LocalizedString::en_gb`], the English version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub struct ExtendedLocalizedString {
    /// English, Great Britain
    pub en_gb: String,
    /// Korean, Korea
    pub ko_kr: String,
    /// French, France
    pub fr_fr: String,
    /// German, Germany
    pub de_de: String,
    /// English, China
    pub en_cn: String,
    /// English, Taiwan
    pub en_tw: String,
    /// Spanish, Spain
    pub es_es: String,
    /// Spanish, Mexico
    pub es_mx: String,
    /// Russian, Russia
    pub ru_ru: String,
    /// Japanese, Japan
    pub ja_jp: String,
    /// Portuguese, Portugal
    ///
    /// Also works as Portuguese, Brazil
    pub pt_pt: String,
    /// Italian, Italy
    pub it_it: String,
    /// Possibly unused.
    pub unknown_12: String,
    /// Possibly unused.
    pub unknown_13: String,
    /// Possibly unused.
    pub unknown_14: String,
    /// Possibly unused.
    pub unknown_15: String,

    /// Unknown flags.
    pub flags: u32,
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
impl ExtendedLocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        en_gb: String,
        ko_kr: String,
        fr_fr: String,
        de_de: String,
        en_cn: String,
        en_tw: String,
        es_es: String,
        es_mx: String,
        ru_ru: String,
        ja_jp: String,
        pt_pt: String,
        it_it: String,
        unknown_12: String,
        unknown_13: String,
        unknown_14: String,
        unknown_15: String,
        flags: u32,
    ) -> Self {
        Self {
            en_gb,
            ko_kr,
            fr_fr,
            de_de,
            en_cn,
            en_tw,
            es_es,
            es_mx,
            ru_ru,
            ja_jp,
            pt_pt,
            it_it,
            unknown_12,
            unknown_13,
            unknown_14,
            unknown_15,
            flags,
        }
    }

    pub(crate) fn string_indices_as_array(&self, string_index: &mut usize) -> [u8; 16 * 4 + 4] {
        let mut arr = [0_u8; 16 * 4 + 4]; // 16 strings of 4 bytes + 4 bytes of flags
        let mut index = 0;

        for s in self.strings() {
            let value = (if !s.is_empty() {
                let v = *string_index;
                *string_index += s.len() + 1;
                v
            } else {
                0
            } as u32)
                .to_le_bytes();

            arr[index] = value[0];
            arr[index + 1] = value[1];
            arr[index + 2] = value[2];
            arr[index + 3] = value[3];
            index += 4;
        }

        let value = &self.flags.to_le_bytes();
        arr[index] = value[0];
        arr[index + 1] = value[1];
        arr[index + 2] = value[2];
        arr[index + 3] = value[3];

        arr
    }

    pub(crate) fn string_block_as_array(
        &self,
        b: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        for s in self.strings() {
            if !s.is_empty() {
                b.write_all(s.as_bytes())?;
                b.write_all(&[0])?;
            };
        }

        Ok(())
    }

    pub(crate) fn string_block_size(&self) -> usize {
        let mut sum = 0;

        for s in self.strings() {
            if !s.is_empty() {
                sum += s.len() + 1;
            }
        }

        sum
    }

    pub(crate) const fn strings(&self) -> [&String; 16] {
        [
            &self.en_gb,
            &self.ko_kr,
            &self.fr_fr,
            &self.de_de,
            &self.en_cn,
            &self.en_tw,
            &self.es_es,
            &self.es_mx,
            &self.ru_ru,
            &self.ja_jp,
            &self.pt_pt,
            &self.it_it,
            &self.unknown_12,
            &self.unknown_13,
            &self.unknown_14,
            &self.unknown_15,
        ]
    }
}

/// DBCs from the English version of the game will only have English version strings, while other localizations will have other languages.
///
/// You are most likely interested in, [`LocalizedString::en_gb`], the English version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(feature = "vanilla")]
pub struct LocalizedString {
    /// English, Great Britain
    pub en_gb: String,
    /// Korean, Korea
    pub ko_kr: String,
    /// French, France
    pub fr_fr: String,
    /// German, Germany
    pub de_de: String,
    /// English, China
    pub en_cn: String,
    /// English, Taiwan
    pub en_tw: String,
    /// Spanish, Spain
    pub es_es: String,
    /// Spanish, Mexico
    pub es_mx: String,
    /// Unknown flags.
    pub flags: u32,
}

#[cfg(feature = "vanilla")]
impl LocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        en_gb: String,
        ko_kr: String,
        fr_fr: String,
        de_de: String,
        en_cn: String,
        en_tw: String,
        es_es: String,
        es_mx: String,
        flags: u32,
    ) -> Self {
        Self {
            en_gb,
            ko_kr,
            fr_fr,
            de_de,
            en_cn,
            en_tw,
            es_es,
            es_mx,
            flags,
        }
    }

    pub(crate) fn string_indices_as_array(&self, string_index: &mut usize) -> [u8; 36] {
        let mut arr = [0_u8; 4 * 8 + 4]; // 8 strings of 4 bytes each, and 4 bytes for flag
        let mut index = 0;

        for s in self.strings() {
            let value = (if !s.is_empty() {
                let v = *string_index;
                *string_index += s.len() + 1;
                v
            } else {
                0
            } as u32)
                .to_le_bytes();

            arr[index] = value[0];
            arr[index + 1] = value[1];
            arr[index + 2] = value[2];
            arr[index + 3] = value[3];
            index += 4;
        }

        let value = &self.flags.to_le_bytes();
        arr[index] = value[0];
        arr[index + 1] = value[1];
        arr[index + 2] = value[2];
        arr[index + 3] = value[3];

        arr
    }

    pub(crate) fn string_block_as_array(
        &self,
        b: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        for s in self.strings() {
            if !s.is_empty() {
                b.write_all(s.as_bytes())?;
                b.write_all(&[0])?;
            };
        }

        Ok(())
    }

    pub(crate) fn string_block_size(&self) -> usize {
        let mut sum = 0;

        for s in self.strings() {
            if !s.is_empty() {
                sum += s.len() + 1;
            }
        }

        sum
    }

    pub(crate) const fn strings(&self) -> [&String; 8] {
        [
            &self.en_gb,
            &self.ko_kr,
            &self.fr_fr,
            &self.de_de,
            &self.en_cn,
            &self.en_tw,
            &self.es_es,
            &self.es_mx,
        ]
    }
}
