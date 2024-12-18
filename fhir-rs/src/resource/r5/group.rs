use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Group {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice="")]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice="")]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business Identifier for this Group
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this group's record is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true)]
    pub active: Option<BooleanDt>,
    /// person | animal | practitioner | device | careteam | healthcareservice | location | organization | relatedperson | specimen
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// definitional | enumerated
    #[fhir(name="membership", min="1", max="1", summary=true, modifier=false, choice="")]
    pub membership: Option<CodeDt>,
    /// Kind of Group members
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Label for Group
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Natural language description of the group
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Number of members
    #[fhir(name="quantity", min="0", max="1", summary=true, modifier=false, choice="")]
    pub quantity: Option<UnsignedIntDt>,
    /// Entity that is the custodian of the Group's definition
    #[fhir(name="managingEntity", min="0", max="1", summary=true, modifier=false, choice="")]
    pub managing_entity: Option<Reference>,
    /// Include / Exclude group members by Trait
    #[fhir(name="characteristic", min="0", max="*", summary=true, modifier=false, choice="")]
    pub characteristic: Option<Vec<GroupCharacteristicBackboneElement>>,
    /// Who or what is in group
    #[fhir(name="member", min="0", max="*", summary=false, modifier=false, choice="")]
    pub member: Option<Vec<GroupMemberBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct GroupMemberBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the group member
    #[fhir(name="entity", min="1", max="1", summary=false, modifier=false, choice="")]
    pub entity: Option<Reference>,
    /// Period member belonged to the group
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
    /// If member is no longer in group
    #[fhir(name="inactive", min="0", max="1", summary=false, modifier=false, choice="")]
    pub inactive: Option<BooleanDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct GroupCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Kind of characteristic
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Value held by characteristic
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<Reference>,
    /// Group includes or excludes
    #[fhir(name="exclude", min="1", max="1", summary=true, modifier=false, choice="")]
    pub exclude: Option<BooleanDt>,
    /// Period over which characteristic is tested
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}

