//! NCBI taxonomy schemas and constants for the `new_taxdump` dataset.

use serde::{Deserialize, Serialize};

pub mod utils;

pub const NCBI_TAXONOMY_BASE_URL: &str = "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/";
pub const NCBI_TAXONOMY_NEW_TAXDUMP_PATH: &str = "new_taxdump/new_taxdump.tar.gz";

#[derive(Debug, Clone, Copy)]
pub struct TaxdumpFileSpec {
    pub file_name: &'static str,
    pub headers: &'static [&'static str],
}

pub const NODES_DMP_HEADERS: &[&str] = &[
    "tax_id",
    "parent_tax_id",
    "rank",
    "embl_code",
    "division_id",
    "inherited_div_flag",
    "genetic_code_id",
    "inherited_gc_flag",
    "mitochondrial_genetic_code_id",
    "inherited_mgc_flag",
    "genbank_hidden_flag",
    "hidden_subtree_root_flag",
    "comments",
    "plastid_genetic_code_id",
    "inherited_pgc_flag",
    "specified_species",
    "hydrogenosome_genetic_code_id",
    "inherited_hgc_flag",
];

pub const NAMES_DMP_HEADERS: &[&str] = &["tax_id", "name_txt", "unique_name", "name_class"];
pub const DELNODES_DMP_HEADERS: &[&str] = &["tax_id"];
pub const MERGED_DMP_HEADERS: &[&str] = &["old_tax_id", "new_tax_id"];
pub const DIVISION_DMP_HEADERS: &[&str] =
    &["division_id", "division_cde", "division_name", "comments"];
pub const GENCODE_DMP_HEADERS: &[&str] =
    &["genetic_code_id", "abbreviation", "name", "cde", "starts"];
pub const CITATIONS_DMP_HEADERS: &[&str] = &[
    "cit_id",
    "cit_key",
    "medline_id",
    "pubmed_id",
    "url",
    "text",
    "taxid_list",
];
pub const TYPE_OF_TYPE_DMP_HEADERS: &[&str] =
    &["type_name", "synonyms", "nomenclature", "description"];
pub const HOST_DMP_HEADERS: &[&str] = &["tax_id", "potential_hosts"];
pub const TYPE_MATERIAL_DMP_HEADERS: &[&str] = &["tax_id", "tax_name", "type", "identifier"];
pub const RANKEDLINEAGE_DMP_HEADERS: &[&str] = &[
    "tax_id",
    "tax_name",
    "species",
    "genus",
    "family",
    "order",
    "class",
    "phylum",
    "kingdom",
    "domain_or_realm",
];
pub const FULLNAMELINEAGE_DMP_HEADERS: &[&str] = &["tax_id", "tax_name", "lineage"];
pub const TAXIDLINEAGE_DMP_HEADERS: &[&str] = &["tax_id", "lineage"];
pub const EXCLUDED_FROM_TYPE_DMP_HEADERS: &[&str] =
    &["tax_id", "tax_name", "property", "voucher_strain"];
pub const IMAGES_DMP_HEADERS: &[&str] = &[
    "image_id",
    "image_key",
    "url",
    "license",
    "attribution",
    "source",
    "properties",
    "taxid_list",
];

pub const NEW_TAXDUMP_FILE_SPECS: &[TaxdumpFileSpec] = &[
    TaxdumpFileSpec {
        file_name: "citations.dmp",
        headers: CITATIONS_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "delnodes.dmp",
        headers: DELNODES_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "division.dmp",
        headers: DIVISION_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "excludedfromtype.dmp",
        headers: EXCLUDED_FROM_TYPE_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "fullnamelineage.dmp",
        headers: FULLNAMELINEAGE_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "gencode.dmp",
        headers: GENCODE_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "host.dmp",
        headers: HOST_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "images.dmp",
        headers: IMAGES_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "merged.dmp",
        headers: MERGED_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "names.dmp",
        headers: NAMES_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "nodes.dmp",
        headers: NODES_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "rankedlineage.dmp",
        headers: RANKEDLINEAGE_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "taxidlineage.dmp",
        headers: TAXIDLINEAGE_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "typematerial.dmp",
        headers: TYPE_MATERIAL_DMP_HEADERS,
    },
    TaxdumpFileSpec {
        file_name: "typeoftype.dmp",
        headers: TYPE_OF_TYPE_DMP_HEADERS,
    },
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodesRecord {
    pub tax_id: u64,
    pub parent_tax_id: u64,
    pub rank: String,
    pub embl_code: String,
    pub division_id: u64,
    pub inherited_div_flag: u8,
    pub genetic_code_id: u64,
    pub inherited_gc_flag: u8,
    pub mitochondrial_genetic_code_id: u64,
    pub inherited_mgc_flag: u8,
    pub genbank_hidden_flag: u8,
    pub hidden_subtree_root_flag: u8,
    pub comments: String,
    pub plastid_genetic_code_id: Option<u64>,
    pub inherited_pgc_flag: Option<u8>,
    pub specified_species: Option<u8>,
    pub hydrogenosome_genetic_code_id: Option<u64>,
    pub inherited_hgc_flag: Option<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NamesRecord {
    pub tax_id: u64,
    pub name_txt: String,
    pub unique_name: String,
    pub name_class: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelNodesRecord {
    pub tax_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MergedRecord {
    pub old_tax_id: u64,
    pub new_tax_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DivisionRecord {
    pub division_id: u64,
    pub division_cde: String,
    pub division_name: String,
    pub comments: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenCodeRecord {
    pub genetic_code_id: u64,
    pub abbreviation: String,
    pub name: String,
    pub cde: String,
    pub starts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CitationsRecord {
    pub cit_id: u64,
    pub cit_key: String,
    pub medline_id: u64,
    pub pubmed_id: u64,
    pub url: String,
    pub text: String,
    pub taxid_list: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeOfTypeRecord {
    pub type_name: String,
    pub synonyms: String,
    pub nomenclature: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeMaterialRecord {
    pub tax_id: u64,
    pub tax_name: String,
    pub r#type: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxIdLineageRecord {
    pub tax_id: u64,
    pub lineage: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RankedLineageRecord {
    pub tax_id: u64,
    pub tax_name: String,
    pub species: String,
    pub genus: String,
    pub family: String,
    pub order: String,
    pub class: String,
    pub phylum: String,
    pub kingdom: String,
    pub domain_or_realm: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FullNameLineageRecord {
    pub tax_id: u64,
    pub tax_name: String,
    pub lineage: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostRecord {
    pub tax_id: u64,
    pub potential_hosts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExcludedFromTypeRecord {
    pub tax_id: u64,
    pub tax_name: String,
    pub property: String,
    pub voucher_strain: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImagesRecord {
    pub image_id: u64,
    pub image_key: String,
    pub url: String,
    pub license: String,
    pub attribution: String,
    pub source: String,
    pub properties: String,
    pub taxid_list: String,
}
