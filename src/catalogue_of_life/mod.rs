//! Catalogue of Life (CoL) Base release constants, artifact specs, and row structs.

use serde::{Deserialize, Serialize};

pub mod utils;

pub const CATALOGUE_OF_LIFE_API_BASE: &str = "https://api.checklistbank.org";
pub const CATALOGUE_OF_LIFE_BASE_DATASET_ID: u64 = 314216;
pub const COL_BASE_EXPORT_FORMAT: &str = "ColDP";
pub const COL_BASE_ARCHIVE_NAME: &str = "col_base_export.zip";

#[derive(Debug, Clone, Copy)]
pub struct ColFileSpec {
    pub file_name: &'static str,
    pub headers: Option<&'static [&'static str]>,
}

pub const COL_BASE_FILE_SPECS: &[ColFileSpec] = &[
    ColFileSpec {
        file_name: "Distribution.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "Media.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "NameRelation.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "NameUsage.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "Reference.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "SpeciesEstimate.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "SpeciesInteraction.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "TaxonConceptRelation.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "TaxonProperty.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "TypeMaterial.tsv",
        headers: None,
    },
    ColFileSpec {
        file_name: "VernacularName.tsv",
        headers: None,
    },
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistributionRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:areaID")]
    pub area_id: Option<String>,
    #[serde(rename = "col:area")]
    pub area: Option<String>,
    #[serde(rename = "col:gazetteer")]
    pub gazetteer: Option<String>,
    #[serde(rename = "col:establishmentMeans")]
    pub establishment_means: Option<String>,
    #[serde(rename = "col:degreeOfEstablishment")]
    pub degree_of_establishment: Option<String>,
    #[serde(rename = "col:pathway")]
    pub pathway: Option<String>,
    #[serde(rename = "col:threatStatus")]
    pub threat_status: Option<String>,
    #[serde(rename = "col:year")]
    pub year: Option<String>,
    #[serde(rename = "col:season")]
    pub season: Option<String>,
    #[serde(rename = "col:lifeStage")]
    pub life_stage: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:url")]
    pub url: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:format")]
    pub format: Option<String>,
    #[serde(rename = "col:title")]
    pub title: Option<String>,
    #[serde(rename = "col:created")]
    pub created: Option<String>,
    #[serde(rename = "col:creator")]
    pub creator: Option<String>,
    #[serde(rename = "col:license")]
    pub license: Option<String>,
    #[serde(rename = "col:link")]
    pub link: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameRelationRecord {
    #[serde(rename = "col:nameID")]
    pub name_id: Option<String>,
    #[serde(rename = "col:relatedNameID")]
    pub related_name_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:page")]
    pub page: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameUsageRecord {
    #[serde(rename = "col:ID")]
    pub id: Option<String>,
    #[serde(rename = "col:alternativeID")]
    pub alternative_id: Option<String>,
    #[serde(rename = "col:nameAlternativeID")]
    pub name_alternative_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:parentID")]
    pub parent_id: Option<String>,
    #[serde(rename = "col:basionymID")]
    pub basionym_id: Option<String>,
    #[serde(rename = "col:status")]
    pub status: Option<String>,
    #[serde(rename = "col:scientificName")]
    pub scientific_name: Option<String>,
    #[serde(rename = "col:authorship")]
    pub authorship: Option<String>,
    #[serde(rename = "col:rank")]
    pub rank: Option<String>,
    #[serde(rename = "col:notho")]
    pub notho: Option<String>,
    #[serde(rename = "col:originalSpelling")]
    pub original_spelling: Option<String>,
    #[serde(rename = "col:uninomial")]
    pub uninomial: Option<String>,
    #[serde(rename = "col:genericName")]
    pub generic_name: Option<String>,
    #[serde(rename = "col:infragenericEpithet")]
    pub infrageneric_epithet: Option<String>,
    #[serde(rename = "col:specificEpithet")]
    pub specific_epithet: Option<String>,
    #[serde(rename = "col:infraspecificEpithet")]
    pub infraspecific_epithet: Option<String>,
    #[serde(rename = "col:cultivarEpithet")]
    pub cultivar_epithet: Option<String>,
    #[serde(rename = "col:combinationAuthorship")]
    pub combination_authorship: Option<String>,
    #[serde(rename = "col:combinationAuthorshipID")]
    pub combination_authorship_id: Option<String>,
    #[serde(rename = "col:combinationExAuthorship")]
    pub combination_ex_authorship: Option<String>,
    #[serde(rename = "col:combinationExAuthorshipID")]
    pub combination_ex_authorship_id: Option<String>,
    #[serde(rename = "col:combinationAuthorshipYear")]
    pub combination_authorship_year: Option<String>,
    #[serde(rename = "col:basionymAuthorship")]
    pub basionym_authorship: Option<String>,
    #[serde(rename = "col:basionymAuthorshipID")]
    pub basionym_authorship_id: Option<String>,
    #[serde(rename = "col:basionymExAuthorship")]
    pub basionym_ex_authorship: Option<String>,
    #[serde(rename = "col:basionymExAuthorshipID")]
    pub basionym_ex_authorship_id: Option<String>,
    #[serde(rename = "col:basionymAuthorshipYear")]
    pub basionym_authorship_year: Option<String>,
    #[serde(rename = "col:namePhrase")]
    pub name_phrase: Option<String>,
    #[serde(rename = "col:nameReferenceID")]
    pub name_reference_id: Option<String>,
    #[serde(rename = "col:publishedInYear")]
    pub published_in_year: Option<String>,
    #[serde(rename = "col:publishedInPage")]
    pub published_in_page: Option<String>,
    #[serde(rename = "col:publishedInPageLink")]
    pub published_in_page_link: Option<String>,
    #[serde(rename = "col:gender")]
    pub gender: Option<String>,
    #[serde(rename = "col:genderAgreement")]
    pub gender_agreement: Option<String>,
    #[serde(rename = "col:etymology")]
    pub etymology: Option<String>,
    #[serde(rename = "col:code")]
    pub code: Option<String>,
    #[serde(rename = "col:nameStatus")]
    pub name_status: Option<String>,
    #[serde(rename = "col:accordingToID")]
    pub according_to_id: Option<String>,
    #[serde(rename = "col:accordingToPage")]
    pub according_to_page: Option<String>,
    #[serde(rename = "col:accordingToPageLink")]
    pub according_to_page_link: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:scrutinizer")]
    pub scrutinizer: Option<String>,
    #[serde(rename = "col:scrutinizerID")]
    pub scrutinizer_id: Option<String>,
    #[serde(rename = "col:scrutinizerDate")]
    pub scrutinizer_date: Option<String>,
    #[serde(rename = "col:extinct")]
    pub extinct: Option<String>,
    #[serde(rename = "col:temporalRangeStart")]
    pub temporal_range_start: Option<String>,
    #[serde(rename = "col:temporalRangeEnd")]
    pub temporal_range_end: Option<String>,
    #[serde(rename = "col:environment")]
    pub environment: Option<String>,
    #[serde(rename = "col:species")]
    pub species: Option<String>,
    #[serde(rename = "col:section")]
    pub section: Option<String>,
    #[serde(rename = "col:subgenus")]
    pub subgenus: Option<String>,
    #[serde(rename = "col:genus")]
    pub genus: Option<String>,
    #[serde(rename = "col:subtribe")]
    pub subtribe: Option<String>,
    #[serde(rename = "col:tribe")]
    pub tribe: Option<String>,
    #[serde(rename = "col:subfamily")]
    pub subfamily: Option<String>,
    #[serde(rename = "col:family")]
    pub family: Option<String>,
    #[serde(rename = "col:superfamily")]
    pub superfamily: Option<String>,
    #[serde(rename = "col:suborder")]
    pub suborder: Option<String>,
    #[serde(rename = "col:order")]
    pub order: Option<String>,
    #[serde(rename = "col:subclass")]
    pub subclass: Option<String>,
    #[serde(rename = "col:class")]
    pub class: Option<String>,
    #[serde(rename = "col:subphylum")]
    pub subphylum: Option<String>,
    #[serde(rename = "col:phylum")]
    pub phylum: Option<String>,
    #[serde(rename = "col:kingdom")]
    pub kingdom: Option<String>,
    #[serde(rename = "col:ordinal")]
    pub ordinal: Option<String>,
    #[serde(rename = "col:branchLength")]
    pub branch_length: Option<String>,
    #[serde(rename = "col:link")]
    pub link: Option<String>,
    #[serde(rename = "col:nameRemarks")]
    pub name_remarks: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReferenceRecord {
    #[serde(rename = "col:ID")]
    pub id: Option<String>,
    #[serde(rename = "col:alternativeID")]
    pub alternative_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:citation")]
    pub citation: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:author")]
    pub author: Option<String>,
    #[serde(rename = "col:editor")]
    pub editor: Option<String>,
    #[serde(rename = "col:title")]
    pub title: Option<String>,
    #[serde(rename = "col:titleShort")]
    pub title_short: Option<String>,
    #[serde(rename = "col:containerAuthor")]
    pub container_author: Option<String>,
    #[serde(rename = "col:containerTitle")]
    pub container_title: Option<String>,
    #[serde(rename = "col:containerTitleShort")]
    pub container_title_short: Option<String>,
    #[serde(rename = "col:issued")]
    pub issued: Option<String>,
    #[serde(rename = "col:accessed")]
    pub accessed: Option<String>,
    #[serde(rename = "col:collectionTitle")]
    pub collection_title: Option<String>,
    #[serde(rename = "col:collectionEditor")]
    pub collection_editor: Option<String>,
    #[serde(rename = "col:volume")]
    pub volume: Option<String>,
    #[serde(rename = "col:issue")]
    pub issue: Option<String>,
    #[serde(rename = "col:edition")]
    pub edition: Option<String>,
    #[serde(rename = "col:page")]
    pub page: Option<String>,
    #[serde(rename = "col:publisher")]
    pub publisher: Option<String>,
    #[serde(rename = "col:publisherPlace")]
    pub publisher_place: Option<String>,
    #[serde(rename = "col:version")]
    pub version: Option<String>,
    #[serde(rename = "col:isbn")]
    pub isbn: Option<String>,
    #[serde(rename = "col:issn")]
    pub issn: Option<String>,
    #[serde(rename = "col:doi")]
    pub doi: Option<String>,
    #[serde(rename = "col:link")]
    pub link: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesEstimateRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:estimate")]
    pub estimate: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesInteractionRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:relatedTaxonID")]
    pub related_taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:relatedTaxonScientificName")]
    pub related_taxon_scientific_name: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxonConceptRelationRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:relatedTaxonID")]
    pub related_taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:type")]
    pub r#type: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxonPropertyRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:property")]
    pub property: Option<String>,
    #[serde(rename = "col:value")]
    pub value: Option<String>,
    #[serde(rename = "col:ordinal")]
    pub ordinal: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:page")]
    pub page: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeMaterialRecord {
    #[serde(rename = "col:ID")]
    pub id: Option<String>,
    #[serde(rename = "col:nameID")]
    pub name_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:citation")]
    pub citation: Option<String>,
    #[serde(rename = "col:status")]
    pub status: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:page")]
    pub page: Option<String>,
    #[serde(rename = "col:country")]
    pub country: Option<String>,
    #[serde(rename = "col:locality")]
    pub locality: Option<String>,
    #[serde(rename = "col:latitude")]
    pub latitude: Option<String>,
    #[serde(rename = "col:longitude")]
    pub longitude: Option<String>,
    #[serde(rename = "col:altitude")]
    pub altitude: Option<String>,
    #[serde(rename = "col:sex")]
    pub sex: Option<String>,
    #[serde(rename = "col:host")]
    pub host: Option<String>,
    #[serde(rename = "col:associatedSequences")]
    pub associated_sequences: Option<String>,
    #[serde(rename = "col:date")]
    pub date: Option<String>,
    #[serde(rename = "col:collector")]
    pub collector: Option<String>,
    #[serde(rename = "col:institutionCode")]
    pub institution_code: Option<String>,
    #[serde(rename = "col:catalogNumber")]
    pub catalog_number: Option<String>,
    #[serde(rename = "col:link")]
    pub link: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VernacularNameRecord {
    #[serde(rename = "col:taxonID")]
    pub taxon_id: Option<String>,
    #[serde(rename = "col:sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "col:name")]
    pub name: Option<String>,
    #[serde(rename = "col:transliteration")]
    pub transliteration: Option<String>,
    #[serde(rename = "col:language")]
    pub language: Option<String>,
    #[serde(rename = "col:preferred")]
    pub preferred: Option<String>,
    #[serde(rename = "col:country")]
    pub country: Option<String>,
    #[serde(rename = "col:area")]
    pub area: Option<String>,
    #[serde(rename = "col:sex")]
    pub sex: Option<String>,
    #[serde(rename = "col:referenceID")]
    pub reference_id: Option<String>,
    #[serde(rename = "col:remarks")]
    pub remarks: Option<String>,
    #[serde(rename = "col:modified")]
    pub modified: Option<String>,
    #[serde(rename = "col:modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "clb:merged")]
    pub merged: Option<String>,
}
