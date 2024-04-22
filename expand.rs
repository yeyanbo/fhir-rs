mod patient {
    use crate::prelude::*;
    #[fhir(base = "DomainResource")]
    pub struct Patient {
        /// Logical id of this artifact
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub id: Option<Id>,
        /// Metadata about the resource
        #[fhir(
            name = "meta",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub meta: Option<Meta>,
        /// A set of rules under which this content was created
        #[fhir(
            name = "implicitRules",
            min = "0",
            max = "1",
            summary = true,
            modifier = true
        )]
        pub implicit_rules: Option<UriDt>,
        /// Language of the resource content
        #[fhir(
            name = "language",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub language: Option<CodeDt>,
        /// Text summary of the resource, for human interpretation
        #[fhir(
            name = "text",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub text: Option<Narrative>,
        /// Contained, inline Resources
        #[fhir(
            name = "contained",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub contained: Option<Vec<AnyResource>>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// An identifier for this patient
        #[fhir(
            name = "identifier",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub identifier: Option<Vec<Identifier>>,
        /// Whether this patient's record is in active use
        #[fhir(
            name = "active",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = false
        )]
        pub active: Option<BooleanDt>,
        /// A name associated with the patient
        #[fhir(
            name = "name",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub name: Option<Vec<HumanName>>,
        /// A contact detail for the individual
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub gender: Option<CodeDt>,
        /// The date of birth for the individual
        #[fhir(
            name = "birthDate",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub birth_date: Option<DateDt>,
        /// Indicates if the individual is deceased or not
        #[fhir(
            name = "deceased",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = true
        )]
        pub deceased: Option<AnyType>,
        /// An address for the individual
        #[fhir(
            name = "address",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub address: Option<Vec<Address>>,
        /// Marital (civil) status of a patient
        #[fhir(
            name = "maritalStatus",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub marital_status: Option<CodeableConcept>,
        /// Whether patient is part of a multiple birth
        #[fhir(
            name = "multipleBirth",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub multiple_birth: Option<IntegerDt>,
        /// Image of the patient
        #[fhir(
            name = "photo",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub photo: Option<Vec<Attachment>>,
        /// A contact party (e.g. guardian, partner, friend) for the patient
        #[fhir(
            name = "contact",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub contact: Option<Vec<PatientContactBackboneElement>>,
        /// A language which may be used to communicate with the patient about his or her health
        #[fhir(
            name = "communication",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub communication: Option<Vec<PatientCommunicationBackboneElement>>,
        /// Patient's nominated primary care provider
        #[fhir(
            name = "generalPractitioner",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub general_practitioner: Option<Vec<Reference>>,
        /// Organization that is the custodian of the patient record
        #[fhir(
            name = "managingOrganization",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub managing_organization: Option<Reference>,
        /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
        #[fhir(
            name = "link",
            min = "0",
            max = "*",
            summary = true,
            modifier = true,
            choice = false
        )]
        pub link: Option<Vec<PatientLinkBackboneElement>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Patient {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "meta",
                "implicit_rules",
                "language",
                "text",
                "contained",
                "extension",
                "modifier_extension",
                "identifier",
                "active",
                "name",
                "telecom",
                "gender",
                "birth_date",
                "deceased",
                "address",
                "marital_status",
                "multiple_birth",
                "photo",
                "contact",
                "communication",
                "general_practitioner",
                "managing_organization",
                "link",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.meta,
                &self.implicit_rules,
                &self.language,
                &self.text,
                &self.contained,
                &self.extension,
                &self.modifier_extension,
                &self.identifier,
                &self.active,
                &self.name,
                &self.telecom,
                &self.gender,
                &self.birth_date,
                &self.deceased,
                &self.address,
                &self.marital_status,
                &self.multiple_birth,
                &self.photo,
                &self.contact,
                &self.communication,
                &self.general_practitioner,
                &self.managing_organization,
                &&self.link,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Patient",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Patient {
        #[inline]
        fn clone(&self) -> Patient {
            Patient {
                id: ::core::clone::Clone::clone(&self.id),
                meta: ::core::clone::Clone::clone(&self.meta),
                implicit_rules: ::core::clone::Clone::clone(&self.implicit_rules),
                language: ::core::clone::Clone::clone(&self.language),
                text: ::core::clone::Clone::clone(&self.text),
                contained: ::core::clone::Clone::clone(&self.contained),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                identifier: ::core::clone::Clone::clone(&self.identifier),
                active: ::core::clone::Clone::clone(&self.active),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                gender: ::core::clone::Clone::clone(&self.gender),
                birth_date: ::core::clone::Clone::clone(&self.birth_date),
                deceased: ::core::clone::Clone::clone(&self.deceased),
                address: ::core::clone::Clone::clone(&self.address),
                marital_status: ::core::clone::Clone::clone(&self.marital_status),
                multiple_birth: ::core::clone::Clone::clone(&self.multiple_birth),
                photo: ::core::clone::Clone::clone(&self.photo),
                contact: ::core::clone::Clone::clone(&self.contact),
                communication: ::core::clone::Clone::clone(&self.communication),
                general_practitioner: ::core::clone::Clone::clone(
                    &self.general_practitioner,
                ),
                managing_organization: ::core::clone::Clone::clone(
                    &self.managing_organization,
                ),
                link: ::core::clone::Clone::clone(&self.link),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Patient {
        #[inline]
        fn default() -> Patient {
            Patient {
                id: ::core::default::Default::default(),
                meta: ::core::default::Default::default(),
                implicit_rules: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                contained: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                identifier: ::core::default::Default::default(),
                active: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                birth_date: ::core::default::Default::default(),
                deceased: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                marital_status: ::core::default::Default::default(),
                multiple_birth: ::core::default::Default::default(),
                photo: ::core::default::Default::default(),
                contact: ::core::default::Default::default(),
                communication: ::core::default::Default::default(),
                general_practitioner: ::core::default::Default::default(),
                managing_organization: ::core::default::Default::default(),
                link: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientLinkBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The other patient or related person resource that the link refers to
        #[fhir(
            name = "other",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub other: Option<Reference>,
        /// replaced-by | replaces | refer | seealso
        #[fhir(
            name = "type",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub type_: Option<CodeDt>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientLinkBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientLinkBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "other",
                &self.other,
                "type_",
                &&self.type_,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientLinkBackboneElement {
        #[inline]
        fn clone(&self) -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                other: ::core::clone::Clone::clone(&self.other),
                type_: ::core::clone::Clone::clone(&self.type_),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientLinkBackboneElement {
        #[inline]
        fn default() -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                other: ::core::default::Default::default(),
                type_: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientContactBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The kind of relationship
        #[fhir(
            name = "relationship",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub relationship: Option<Vec<CodeableConcept>>,
        /// A name associated with the contact person
        #[fhir(
            name = "name",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub name: Option<HumanName>,
        /// A contact detail for the person
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// Address for the contact person
        #[fhir(
            name = "address",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub address: Option<Address>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub gender: Option<CodeDt>,
        /// Organization that is associated with the contact
        #[fhir(
            name = "organization",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub organization: Option<Reference>,
        /// The period during which this contact person or organization is valid to be contacted relating to this patient
        #[fhir(
            name = "period",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub period: Option<Period>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientContactBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "extension",
                "modifier_extension",
                "relationship",
                "name",
                "telecom",
                "address",
                "gender",
                "organization",
                "period",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.extension,
                &self.modifier_extension,
                &self.relationship,
                &self.name,
                &self.telecom,
                &self.address,
                &self.gender,
                &self.organization,
                &&self.period,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "PatientContactBackboneElement",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientContactBackboneElement {
        #[inline]
        fn clone(&self) -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                relationship: ::core::clone::Clone::clone(&self.relationship),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                address: ::core::clone::Clone::clone(&self.address),
                gender: ::core::clone::Clone::clone(&self.gender),
                organization: ::core::clone::Clone::clone(&self.organization),
                period: ::core::clone::Clone::clone(&self.period),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientContactBackboneElement {
        #[inline]
        fn default() -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                relationship: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                organization: ::core::default::Default::default(),
                period: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientCommunicationBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The language which can be used to communicate with the patient about his or her health
        #[fhir(
            name = "language",
            min = "1",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub language: Option<CodeableConcept>,
        /// Language preference indicator
        #[fhir(
            name = "preferred",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub preferred: Option<BooleanDt>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientCommunicationBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientCommunicationBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "language",
                &self.language,
                "preferred",
                &&self.preferred,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientCommunicationBackboneElement {
        #[inline]
        fn clone(&self) -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                language: ::core::clone::Clone::clone(&self.language),
                preferred: ::core::clone::Clone::clone(&self.preferred),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientCommunicationBackboneElement {
        #[inline]
        fn default() -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                preferred: ::core::default::Default::default(),
            }
        }
    }
}
mod patient {
    use crate::prelude::*;
    #[fhir(base = "DomainResource")]
    pub struct Patient {
        /// Logical id of this artifact
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub id: Option<Id>,
        /// Metadata about the resource
        #[fhir(
            name = "meta",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub meta: Option<Meta>,
        /// A set of rules under which this content was created
        #[fhir(
            name = "implicitRules",
            min = "0",
            max = "1",
            summary = true,
            modifier = true
        )]
        pub implicit_rules: Option<UriDt>,
        /// Language of the resource content
        #[fhir(
            name = "language",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub language: Option<CodeDt>,
        /// Text summary of the resource, for human interpretation
        #[fhir(
            name = "text",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub text: Option<Narrative>,
        /// Contained, inline Resources
        #[fhir(
            name = "contained",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub contained: Option<Vec<AnyResource>>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// An identifier for this patient
        #[fhir(
            name = "identifier",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub identifier: Option<Vec<Identifier>>,
        /// Whether this patient's record is in active use
        #[fhir(
            name = "active",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = false
        )]
        pub active: Option<BooleanDt>,
        /// A name associated with the patient
        #[fhir(
            name = "name",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub name: Option<Vec<HumanName>>,
        /// A contact detail for the individual
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub gender: Option<CodeDt>,
        /// The date of birth for the individual
        #[fhir(
            name = "birthDate",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub birth_date: Option<DateDt>,
        /// Indicates if the individual is deceased or not
        #[fhir(
            name = "deceased",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = true
        )]
        pub deceased: Option<AnyType>,
        /// An address for the individual
        #[fhir(
            name = "address",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub address: Option<Vec<Address>>,
        /// Marital (civil) status of a patient
        #[fhir(
            name = "maritalStatus",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub marital_status: Option<CodeableConcept>,
        /// Whether patient is part of a multiple birth
        #[fhir(
            name = "multipleBirth",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub multiple_birth: Option<IntegerDt>,
        /// Image of the patient
        #[fhir(
            name = "photo",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub photo: Option<Vec<Attachment>>,
        /// A contact party (e.g. guardian, partner, friend) for the patient
        #[fhir(
            name = "contact",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub contact: Option<Vec<PatientContactBackboneElement>>,
        /// A language which may be used to communicate with the patient about his or her health
        #[fhir(
            name = "communication",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub communication: Option<Vec<PatientCommunicationBackboneElement>>,
        /// Patient's nominated primary care provider
        #[fhir(
            name = "generalPractitioner",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub general_practitioner: Option<Vec<Reference>>,
        /// Organization that is the custodian of the patient record
        #[fhir(
            name = "managingOrganization",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub managing_organization: Option<Reference>,
        /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
        #[fhir(
            name = "link",
            min = "0",
            max = "*",
            summary = true,
            modifier = true,
            choice = false
        )]
        pub link: Option<Vec<PatientLinkBackboneElement>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Patient {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "meta",
                "implicit_rules",
                "language",
                "text",
                "contained",
                "extension",
                "modifier_extension",
                "identifier",
                "active",
                "name",
                "telecom",
                "gender",
                "birth_date",
                "deceased",
                "address",
                "marital_status",
                "multiple_birth",
                "photo",
                "contact",
                "communication",
                "general_practitioner",
                "managing_organization",
                "link",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.meta,
                &self.implicit_rules,
                &self.language,
                &self.text,
                &self.contained,
                &self.extension,
                &self.modifier_extension,
                &self.identifier,
                &self.active,
                &self.name,
                &self.telecom,
                &self.gender,
                &self.birth_date,
                &self.deceased,
                &self.address,
                &self.marital_status,
                &self.multiple_birth,
                &self.photo,
                &self.contact,
                &self.communication,
                &self.general_practitioner,
                &self.managing_organization,
                &&self.link,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Patient",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Patient {
        #[inline]
        fn clone(&self) -> Patient {
            Patient {
                id: ::core::clone::Clone::clone(&self.id),
                meta: ::core::clone::Clone::clone(&self.meta),
                implicit_rules: ::core::clone::Clone::clone(&self.implicit_rules),
                language: ::core::clone::Clone::clone(&self.language),
                text: ::core::clone::Clone::clone(&self.text),
                contained: ::core::clone::Clone::clone(&self.contained),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                identifier: ::core::clone::Clone::clone(&self.identifier),
                active: ::core::clone::Clone::clone(&self.active),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                gender: ::core::clone::Clone::clone(&self.gender),
                birth_date: ::core::clone::Clone::clone(&self.birth_date),
                deceased: ::core::clone::Clone::clone(&self.deceased),
                address: ::core::clone::Clone::clone(&self.address),
                marital_status: ::core::clone::Clone::clone(&self.marital_status),
                multiple_birth: ::core::clone::Clone::clone(&self.multiple_birth),
                photo: ::core::clone::Clone::clone(&self.photo),
                contact: ::core::clone::Clone::clone(&self.contact),
                communication: ::core::clone::Clone::clone(&self.communication),
                general_practitioner: ::core::clone::Clone::clone(
                    &self.general_practitioner,
                ),
                managing_organization: ::core::clone::Clone::clone(
                    &self.managing_organization,
                ),
                link: ::core::clone::Clone::clone(&self.link),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Patient {
        #[inline]
        fn default() -> Patient {
            Patient {
                id: ::core::default::Default::default(),
                meta: ::core::default::Default::default(),
                implicit_rules: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                contained: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                identifier: ::core::default::Default::default(),
                active: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                birth_date: ::core::default::Default::default(),
                deceased: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                marital_status: ::core::default::Default::default(),
                multiple_birth: ::core::default::Default::default(),
                photo: ::core::default::Default::default(),
                contact: ::core::default::Default::default(),
                communication: ::core::default::Default::default(),
                general_practitioner: ::core::default::Default::default(),
                managing_organization: ::core::default::Default::default(),
                link: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientLinkBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The other patient or related person resource that the link refers to
        #[fhir(
            name = "other",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub other: Option<Reference>,
        /// replaced-by | replaces | refer | seealso
        #[fhir(
            name = "type",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = false
        )]
        pub type_: Option<CodeDt>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientLinkBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientLinkBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "other",
                &self.other,
                "type_",
                &&self.type_,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientLinkBackboneElement {
        #[inline]
        fn clone(&self) -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                other: ::core::clone::Clone::clone(&self.other),
                type_: ::core::clone::Clone::clone(&self.type_),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientLinkBackboneElement {
        #[inline]
        fn default() -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                other: ::core::default::Default::default(),
                type_: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientContactBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The kind of relationship
        #[fhir(
            name = "relationship",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub relationship: Option<Vec<CodeableConcept>>,
        /// A name associated with the contact person
        #[fhir(
            name = "name",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub name: Option<HumanName>,
        /// A contact detail for the person
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// Address for the contact person
        #[fhir(
            name = "address",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub address: Option<Address>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub gender: Option<CodeDt>,
        /// Organization that is associated with the contact
        #[fhir(
            name = "organization",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub organization: Option<Reference>,
        /// The period during which this contact person or organization is valid to be contacted relating to this patient
        #[fhir(
            name = "period",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub period: Option<Period>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientContactBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "extension",
                "modifier_extension",
                "relationship",
                "name",
                "telecom",
                "address",
                "gender",
                "organization",
                "period",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.extension,
                &self.modifier_extension,
                &self.relationship,
                &self.name,
                &self.telecom,
                &self.address,
                &self.gender,
                &self.organization,
                &&self.period,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "PatientContactBackboneElement",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientContactBackboneElement {
        #[inline]
        fn clone(&self) -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                relationship: ::core::clone::Clone::clone(&self.relationship),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                address: ::core::clone::Clone::clone(&self.address),
                gender: ::core::clone::Clone::clone(&self.gender),
                organization: ::core::clone::Clone::clone(&self.organization),
                period: ::core::clone::Clone::clone(&self.period),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientContactBackboneElement {
        #[inline]
        fn default() -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                relationship: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                organization: ::core::default::Default::default(),
                period: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientCommunicationBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The language which can be used to communicate with the patient about his or her health
        #[fhir(
            name = "language",
            min = "1",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub language: Option<CodeableConcept>,
        /// Language preference indicator
        #[fhir(
            name = "preferred",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = false
        )]
        pub preferred: Option<BooleanDt>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientCommunicationBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientCommunicationBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "language",
                &self.language,
                "preferred",
                &&self.preferred,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientCommunicationBackboneElement {
        #[inline]
        fn clone(&self) -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                language: ::core::clone::Clone::clone(&self.language),
                preferred: ::core::clone::Clone::clone(&self.preferred),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientCommunicationBackboneElement {
        #[inline]
        fn default() -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                preferred: ::core::default::Default::default(),
            }
        }
    }
}
mod patient {
    use crate::prelude::*;
    #[fhir(base = "DomainResource")]
    pub struct Patient {
        /// Logical id of this artifact
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub id: Option<Id>,
        /// Metadata about the resource
        #[fhir(
            name = "meta",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub meta: Option<Meta>,
        /// A set of rules under which this content was created
        #[fhir(
            name = "implicitRules",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = ""
        )]
        pub implicit_rules: Option<UriDt>,
        /// Language of the resource content
        #[fhir(
            name = "language",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub language: Option<CodeDt>,
        /// Text summary of the resource, for human interpretation
        #[fhir(
            name = "text",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub text: Option<Narrative>,
        /// Contained, inline Resources
        #[fhir(
            name = "contained",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub contained: Option<Vec<AnyResource>>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// An identifier for this patient
        #[fhir(
            name = "identifier",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub identifier: Option<Vec<Identifier>>,
        /// Whether this patient's record is in active use
        #[fhir(
            name = "active",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = ""
        )]
        pub active: Option<BooleanDt>,
        /// A name associated with the patient
        #[fhir(
            name = "name",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub name: Option<Vec<HumanName>>,
        /// A contact detail for the individual
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub gender: Option<CodeDt>,
        /// The date of birth for the individual
        #[fhir(
            name = "birthDate",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub birth_date: Option<DateDt>,
        /// Indicates if the individual is deceased or not
        #[fhir(
            name = "deceased",
            min = "0",
            max = "1",
            summary = true,
            modifier = true,
            choice = "Boolean|DateTime"
        )]
        pub deceased: Option<AnyType>,
        /// An address for the individual
        #[fhir(
            name = "address",
            min = "0",
            max = "*",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub address: Option<Vec<Address>>,
        /// Marital (civil) status of a patient
        #[fhir(
            name = "maritalStatus",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub marital_status: Option<CodeableConcept>,
        /// Whether patient is part of a multiple birth
        #[fhir(
            name = "multipleBirth",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub multiple_birth: Option<IntegerDt>,
        /// Image of the patient
        #[fhir(
            name = "photo",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub photo: Option<Vec<Attachment>>,
        /// A contact party (e.g. guardian, partner, friend) for the patient
        #[fhir(
            name = "contact",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub contact: Option<Vec<PatientContactBackboneElement>>,
        /// A language which may be used to communicate with the patient about his or her health
        #[fhir(
            name = "communication",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub communication: Option<Vec<PatientCommunicationBackboneElement>>,
        /// Patient's nominated primary care provider
        #[fhir(
            name = "generalPractitioner",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub general_practitioner: Option<Vec<Reference>>,
        /// Organization that is the custodian of the patient record
        #[fhir(
            name = "managingOrganization",
            min = "0",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub managing_organization: Option<Reference>,
        /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
        #[fhir(
            name = "link",
            min = "0",
            max = "*",
            summary = true,
            modifier = true,
            choice = ""
        )]
        pub link: Option<Vec<PatientLinkBackboneElement>>,
    }
    impl Resource for Patient {
        fn resource_name(&self) -> String {
            "Patient".to_string()
        }
        fn id(&self) -> &Option<String> {
            &self.id
        }
        fn set_id<T: Into<Id>>(mut self, id: T) -> Self {
            self.id = Some(id.into());
            self
        }
        fn meta(&self) -> &Option<Meta> {
            &self.meta
        }
        fn set_meta(mut self, meta: Meta) -> Self {
            self.meta = Some(meta);
            self
        }
        fn assert(&self, path: String) -> Result<bool> {
            let mut paths = FhirPaths::parse(path)?;
            match paths.response() {
                Some(response) => {
                    if response != &FunctionResponse::Bool {
                        return Err(
                            FhirError::error(
                                "该表达式不是一个有效的路径表达式，最后返回值不是Boolean",
                            ),
                        );
                    }
                }
                None => {
                    return Err(
                        FhirError::error(
                            "该表达式不是一个有效的路径表达式，最后返回值不是Boolean",
                        ),
                    );
                }
            }
            if let Some(first) = paths.next() {
                if !first.match_resource_type_name(&self.resource_name()) {
                    return Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "路径中首个组成与当前资源类型[{0}]不符",
                                    self.resource_name(),
                                ),
                            );
                            res
                        }),
                    );
                }
            }
            let mut vv = self.as_collection2();
            while let Some(func) = paths.next() {
                vv = vv.exec(&func, &mut paths)?;
                {
                    ::std::io::_print(format_args!("Response: {0:?}\n", &vv));
                }
            }
            match vv {
                PathResponse::Bool(value) => Ok(value),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn path(&self, path: String) -> Result<Collection> {
            let mut paths = FhirPaths::parse(path)?;
            match paths.response() {
                Some(response) => {
                    if response != &FunctionResponse::Collection {
                        return Err(
                            FhirError::error(
                                "该表达式不是一个有效的路径表达式，最后返回值不是Collection",
                            ),
                        );
                    }
                }
                None => {
                    return Err(
                        FhirError::error(
                            "该表达式不是一个有效的路径表达式，最后返回值不是Collection",
                        ),
                    );
                }
            }
            if let Some(first) = paths.next() {
                if !first.match_resource_type_name(&self.resource_name()) {
                    return Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "路径中首个组成与当前资源类型[{0}]不符",
                                    self.resource_name(),
                                ),
                            );
                            res
                        }),
                    );
                }
            }
            let mut vv = self.as_collection2();
            while let Some(func) = paths.next() {
                vv = vv.exec(&func, &mut paths)?;
                {
                    ::std::io::_print(format_args!("Response: {0:?}\n", &vv));
                }
            }
            match vv {
                PathResponse::Collection(collection) => Ok(collection),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl Base for Patient {}
    impl DomainResource for Patient {
        fn extension(&self) -> &Option<Vec<Extension>> {
            &self.extension
        }
        fn set_extension(mut self, ext: Vec<Extension>) -> Self {
            self.extension = Some(ext);
            self
        }
        fn add_extension(mut self, ext: Extension) -> Self {
            match self.extension {
                Some(ref mut exts) => {
                    exts.push(ext);
                }
                None => {
                    self
                        .extension = Some(
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([ext])),
                    );
                }
            }
            self
        }
        fn modifier_extension(&self) -> &Option<Vec<Extension>> {
            &self.modifier_extension
        }
        fn set_modifier_extension(mut self, ext: Vec<Extension>) -> Self {
            self.modifier_extension = Some(ext);
            self
        }
        fn add_modifier_extension(mut self, ext: Extension) -> Self {
            match self.modifier_extension {
                Some(ref mut exts) => {
                    exts.push(ext);
                }
                None => {
                    self
                        .modifier_extension = Some(
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([ext])),
                    );
                }
            }
            self
        }
    }
    impl Patient {
        pub fn set_identifier(mut self, v: Vec<Identifier>) -> Self {
            self.identifier = Some(v);
            self
        }
        pub fn add_identifier(mut self, v: Identifier) -> Self {
            match self.identifier {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.identifier = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_active<T: Into<BooleanDt>>(mut self, v: T) -> Self {
            self.active = Some(v.into());
            self
        }
        pub fn set_name(mut self, v: Vec<HumanName>) -> Self {
            self.name = Some(v);
            self
        }
        pub fn add_name(mut self, v: HumanName) -> Self {
            match self.name {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.name = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_telecom(mut self, v: Vec<ContactPoint>) -> Self {
            self.telecom = Some(v);
            self
        }
        pub fn add_telecom(mut self, v: ContactPoint) -> Self {
            match self.telecom {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.telecom = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_gender<T: Into<CodeDt>>(mut self, v: T) -> Self {
            self.gender = Some(v.into());
            self
        }
        pub fn set_birth_date<T: Into<DateDt>>(mut self, v: T) -> Self {
            self.birth_date = Some(v.into());
            self
        }
        pub fn set_deceased(mut self, v: AnyType) -> Self {
            self.deceased = Some(v);
            self
        }
        pub fn set_address(mut self, v: Vec<Address>) -> Self {
            self.address = Some(v);
            self
        }
        pub fn add_address(mut self, v: Address) -> Self {
            match self.address {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.address = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_marital_status(mut self, v: CodeableConcept) -> Self {
            self.marital_status = Some(v);
            self
        }
        pub fn set_multiple_birth<T: Into<IntegerDt>>(mut self, v: T) -> Self {
            self.multiple_birth = Some(v.into());
            self
        }
        pub fn set_photo(mut self, v: Vec<Attachment>) -> Self {
            self.photo = Some(v);
            self
        }
        pub fn add_photo(mut self, v: Attachment) -> Self {
            match self.photo {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.photo = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_contact(mut self, v: Vec<PatientContactBackboneElement>) -> Self {
            self.contact = Some(v);
            self
        }
        pub fn add_contact(mut self, v: PatientContactBackboneElement) -> Self {
            match self.contact {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.contact = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_communication(
            mut self,
            v: Vec<PatientCommunicationBackboneElement>,
        ) -> Self {
            self.communication = Some(v);
            self
        }
        pub fn add_communication(
            mut self,
            v: PatientCommunicationBackboneElement,
        ) -> Self {
            match self.communication {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.communication = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_general_practitioner(mut self, v: Vec<Reference>) -> Self {
            self.general_practitioner = Some(v);
            self
        }
        pub fn add_general_practitioner(mut self, v: Reference) -> Self {
            match self.general_practitioner {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.general_practitioner = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
        pub fn set_managing_organization(mut self, v: Reference) -> Self {
            self.managing_organization = Some(v);
            self
        }
        pub fn set_link(mut self, v: Vec<PatientLinkBackboneElement>) -> Self {
            self.link = Some(v);
            self
        }
        pub fn add_link(mut self, v: PatientLinkBackboneElement) -> Self {
            match self.link {
                None => {
                    let mut vec = Vec::new();
                    vec.push(v);
                    self.link = Some(vec);
                }
                Some(ref mut vec) => {
                    vec.push(v);
                }
            }
            self
        }
    }
    impl Serialize for Patient {
        fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
            let mut serialize_struct = serializer.serialize_resource("Patient")?;
            serialize_struct.serialize_id(&self.id)?;
            serialize_struct.serialize_field("meta", &self.meta)?;
            serialize_struct.serialize_field("implicitRules", &self.implicit_rules)?;
            serialize_struct.serialize_field("language", &self.language)?;
            serialize_struct.serialize_field("text", &self.text)?;
            serialize_struct.serialize_field("contained", &self.contained)?;
            serialize_struct.serialize_field("extension", &self.extension)?;
            serialize_struct
                .serialize_field("modifierExtension", &self.modifier_extension)?;
            serialize_struct.serialize_field("identifier", &self.identifier)?;
            serialize_struct.serialize_field("active", &self.active)?;
            serialize_struct.serialize_field("name", &self.name)?;
            serialize_struct.serialize_field("telecom", &self.telecom)?;
            serialize_struct.serialize_field("gender", &self.gender)?;
            serialize_struct.serialize_field("birthDate", &self.birth_date)?;
            serialize_struct.serialize_field("deceased", &self.deceased)?;
            serialize_struct.serialize_field("address", &self.address)?;
            serialize_struct.serialize_field("maritalStatus", &self.marital_status)?;
            serialize_struct.serialize_field("multipleBirth", &self.multiple_birth)?;
            serialize_struct.serialize_field("photo", &self.photo)?;
            serialize_struct.serialize_field("contact", &self.contact)?;
            serialize_struct.serialize_field("communication", &self.communication)?;
            serialize_struct
                .serialize_field("generalPractitioner", &self.general_practitioner)?;
            serialize_struct
                .serialize_field("managingOrganization", &self.managing_organization)?;
            serialize_struct.serialize_field("link", &self.link)?;
            serialize_struct.serialize_end()
        }
    }
    impl<'de> Deserialize<'de> for Patient {
        fn deserialize<De>(deserializer: De) -> Result<Self>
        where
            De: Deserializer<'de>,
        {
            pub struct PatientVisitor;
            impl<'de> Visitor<'de> for PatientVisitor {
                type Value = Patient;
                fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value>
                where
                    M: MapAccess<'de>,
                {
                    let mut id: Option<StringDt> = None;
                    let mut meta: Option<Meta> = None;
                    let mut implicit_rules: Option<UriDt> = None;
                    let mut language: Option<CodeDt> = None;
                    let mut text: Option<Narrative> = None;
                    let mut contained: Option<Vec<AnyResource>> = None;
                    let mut extension: Option<Vec<Extension>> = None;
                    let mut modifier_extension: Option<Vec<Extension>> = None;
                    let mut identifier: Option<Vec<Identifier>> = None;
                    let mut active: Option<BooleanDt> = None;
                    let mut name: Option<Vec<HumanName>> = None;
                    let mut telecom: Option<Vec<ContactPoint>> = None;
                    let mut gender: Option<CodeDt> = None;
                    let mut birth_date: Option<DateDt> = None;
                    let mut deceased: Option<AnyType> = None;
                    let mut address: Option<Vec<Address>> = None;
                    let mut marital_status: Option<CodeableConcept> = None;
                    let mut multiple_birth: Option<IntegerDt> = None;
                    let mut photo: Option<Vec<Attachment>> = None;
                    let mut contact: Option<Vec<PatientContactBackboneElement>> = None;
                    let mut communication: Option<
                        Vec<PatientCommunicationBackboneElement>,
                    > = None;
                    let mut general_practitioner: Option<Vec<Reference>> = None;
                    let mut managing_organization: Option<Reference> = None;
                    let mut link: Option<Vec<PatientLinkBackboneElement>> = None;
                    while let Some(keys) = mapp.next_key()? {
                        match keys.as_str() {
                            "id" => {
                                id = Some(mapp.next_value()?);
                            }
                            "meta" => {
                                meta = Some(mapp.next_value()?);
                            }
                            "implicitRules" => {
                                implicit_rules = Some(mapp.next_value()?);
                            }
                            "language" => {
                                language = Some(mapp.next_value()?);
                            }
                            "text" => {
                                text = Some(mapp.next_value()?);
                            }
                            "contained" => {
                                contained = Some(mapp.next_value()?);
                            }
                            "extension" => {
                                extension = Some(mapp.next_value()?);
                            }
                            "modifierExtension" => {
                                modifier_extension = Some(mapp.next_value()?);
                            }
                            "identifier" => {
                                identifier = Some(mapp.next_value()?);
                            }
                            "active" => {
                                active = Some(mapp.next_value()?);
                            }
                            "name" => {
                                name = Some(mapp.next_value()?);
                            }
                            "telecom" => {
                                telecom = Some(mapp.next_value()?);
                            }
                            "gender" => {
                                gender = Some(mapp.next_value()?);
                            }
                            "birthDate" => {
                                birth_date = Some(mapp.next_value()?);
                            }
                            k_value if k_value.starts_with("deceased") => {
                                let ttt = k_value.replace("deceased", "");
                                deceased = match ttt.as_str() {
                                    "Boolean" => Some(AnyType::Boolean(mapp.next_value()?)),
                                    "DateTime" => Some(AnyType::DateTime(mapp.next_value()?)),
                                    _ => {
                                        return Err(
                                            FhirError::Message({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("{0}不在可选类型范围之内", &k_value),
                                                );
                                                res
                                            }),
                                        );
                                    }
                                };
                            }
                            "address" => {
                                address = Some(mapp.next_value()?);
                            }
                            "maritalStatus" => {
                                marital_status = Some(mapp.next_value()?);
                            }
                            "multipleBirth" => {
                                multiple_birth = Some(mapp.next_value()?);
                            }
                            "photo" => {
                                photo = Some(mapp.next_value()?);
                            }
                            "contact" => {
                                contact = Some(mapp.next_value()?);
                            }
                            "communication" => {
                                communication = Some(mapp.next_value()?);
                            }
                            "generalPractitioner" => {
                                general_practitioner = Some(mapp.next_value()?);
                            }
                            "managingOrganization" => {
                                managing_organization = Some(mapp.next_value()?);
                            }
                            "link" => {
                                link = Some(mapp.next_value()?);
                            }
                            _ => {
                                return Err(
                                    FhirError::error_string({
                                        let res = ::alloc::fmt::format(
                                            format_args!("读到不存在的键:{0}", keys),
                                        );
                                        res
                                    }),
                                );
                            }
                        }
                    }
                    let id = match id {
                        Some(id) => {
                            match id.value {
                                Some(value) => Some(value),
                                None => None,
                            }
                        }
                        None => None,
                    };
                    Ok(Patient {
                        id,
                        meta,
                        implicit_rules,
                        language,
                        text,
                        contained,
                        extension,
                        modifier_extension,
                        identifier,
                        active,
                        name,
                        telecom,
                        gender,
                        birth_date,
                        deceased,
                        address,
                        marital_status,
                        multiple_birth,
                        photo,
                        contact,
                        communication,
                        general_practitioner,
                        managing_organization,
                        link,
                    })
                }
            }
            deserializer.deserialize_struct("", PatientVisitor)
        }
    }
    impl Executor for Patient {
        fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
            match func.definition.function_name() {
                FunctionName::Element => {
                    match &func.params {
                        FunctionParam::String(name) => {
                            match name.as_str() {
                                "id" => self.id.exec(&func, paths),
                                "meta" => self.meta.exec(&func, paths),
                                "implicitRules" => self.implicit_rules.exec(&func, paths),
                                "language" => self.language.exec(&func, paths),
                                "text" => self.text.exec(&func, paths),
                                "contained" => self.contained.exec(&func, paths),
                                "extension" => self.extension.exec(&func, paths),
                                "modifierExtension" => {
                                    self.modifier_extension.exec(&func, paths)
                                }
                                "identifier" => self.identifier.exec(&func, paths),
                                "active" => self.active.exec(&func, paths),
                                "name" => self.name.exec(&func, paths),
                                "telecom" => self.telecom.exec(&func, paths),
                                "gender" => self.gender.exec(&func, paths),
                                "birthDate" => self.birth_date.exec(&func, paths),
                                "deceased" => self.deceased.exec(&func, paths),
                                "address" => self.address.exec(&func, paths),
                                "maritalStatus" => self.marital_status.exec(&func, paths),
                                "multipleBirth" => self.multiple_birth.exec(&func, paths),
                                "photo" => self.photo.exec(&func, paths),
                                "contact" => self.contact.exec(&func, paths),
                                "communication" => self.communication.exec(&func, paths),
                                "generalPractitioner" => {
                                    self.general_practitioner.exec(&func, paths)
                                }
                                "managingOrganization" => {
                                    self.managing_organization.exec(&func, paths)
                                }
                                "link" => self.link.exec(&func, paths),
                                other => {
                                    Err(
                                        FhirError::Message({
                                            let res = ::alloc::fmt::format(
                                                format_args!("无效的路径名:[{0}]", other),
                                            );
                                            res
                                        }),
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => {
                    Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!("Patient: 无效的函数名:{0:?}", &func),
                            );
                            res
                        }),
                    )
                }
            }
        }
        fn as_collection(&self) -> Collection {
            Collection(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Box::new(self.clone())]),
                ),
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Patient {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "meta",
                "implicit_rules",
                "language",
                "text",
                "contained",
                "extension",
                "modifier_extension",
                "identifier",
                "active",
                "name",
                "telecom",
                "gender",
                "birth_date",
                "deceased",
                "address",
                "marital_status",
                "multiple_birth",
                "photo",
                "contact",
                "communication",
                "general_practitioner",
                "managing_organization",
                "link",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.meta,
                &self.implicit_rules,
                &self.language,
                &self.text,
                &self.contained,
                &self.extension,
                &self.modifier_extension,
                &self.identifier,
                &self.active,
                &self.name,
                &self.telecom,
                &self.gender,
                &self.birth_date,
                &self.deceased,
                &self.address,
                &self.marital_status,
                &self.multiple_birth,
                &self.photo,
                &self.contact,
                &self.communication,
                &self.general_practitioner,
                &self.managing_organization,
                &&self.link,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Patient",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Patient {
        #[inline]
        fn clone(&self) -> Patient {
            Patient {
                id: ::core::clone::Clone::clone(&self.id),
                meta: ::core::clone::Clone::clone(&self.meta),
                implicit_rules: ::core::clone::Clone::clone(&self.implicit_rules),
                language: ::core::clone::Clone::clone(&self.language),
                text: ::core::clone::Clone::clone(&self.text),
                contained: ::core::clone::Clone::clone(&self.contained),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                identifier: ::core::clone::Clone::clone(&self.identifier),
                active: ::core::clone::Clone::clone(&self.active),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                gender: ::core::clone::Clone::clone(&self.gender),
                birth_date: ::core::clone::Clone::clone(&self.birth_date),
                deceased: ::core::clone::Clone::clone(&self.deceased),
                address: ::core::clone::Clone::clone(&self.address),
                marital_status: ::core::clone::Clone::clone(&self.marital_status),
                multiple_birth: ::core::clone::Clone::clone(&self.multiple_birth),
                photo: ::core::clone::Clone::clone(&self.photo),
                contact: ::core::clone::Clone::clone(&self.contact),
                communication: ::core::clone::Clone::clone(&self.communication),
                general_practitioner: ::core::clone::Clone::clone(
                    &self.general_practitioner,
                ),
                managing_organization: ::core::clone::Clone::clone(
                    &self.managing_organization,
                ),
                link: ::core::clone::Clone::clone(&self.link),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Patient {
        #[inline]
        fn default() -> Patient {
            Patient {
                id: ::core::default::Default::default(),
                meta: ::core::default::Default::default(),
                implicit_rules: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                contained: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                identifier: ::core::default::Default::default(),
                active: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                birth_date: ::core::default::Default::default(),
                deceased: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                marital_status: ::core::default::Default::default(),
                multiple_birth: ::core::default::Default::default(),
                photo: ::core::default::Default::default(),
                contact: ::core::default::Default::default(),
                communication: ::core::default::Default::default(),
                general_practitioner: ::core::default::Default::default(),
                managing_organization: ::core::default::Default::default(),
                link: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientLinkBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The other patient or related person resource that the link refers to
        #[fhir(
            name = "other",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub other: Option<Reference>,
        /// replaced-by | replaces | refer | seealso
        #[fhir(
            name = "type",
            min = "1",
            max = "1",
            summary = true,
            modifier = false,
            choice = ""
        )]
        pub type_: Option<CodeDt>,
    }
    impl Base for PatientLinkBackboneElement {}
    impl Serialize for PatientLinkBackboneElement {
        fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
            let mut serialize_struct = serializer.serialize_struct("")?;
            serialize_struct.serialize_id(&self.id)?;
            serialize_struct.serialize_extension(&self.extension)?;
            serialize_struct
                .serialize_field("modifierExtension", &self.modifier_extension)?;
            serialize_struct.serialize_field("other", &self.other)?;
            serialize_struct.serialize_field("type", &self.type_)?;
            serialize_struct.serialize_end()
        }
    }
    impl<'de> Deserialize<'de> for PatientLinkBackboneElement {
        fn deserialize<De>(deserializer: De) -> Result<Self>
        where
            De: Deserializer<'de>,
        {
            pub struct PatientLinkBackboneElementVisitor;
            impl<'de> Visitor<'de> for PatientLinkBackboneElementVisitor {
                type Value = PatientLinkBackboneElement;
                fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value>
                where
                    M: MapAccess<'de>,
                {
                    let mut id: Option<String> = None;
                    let mut extension: Option<Vec<Extension>> = None;
                    let mut modifier_extension: Option<Vec<Extension>> = None;
                    let mut other: Option<Reference> = None;
                    let mut type_: Option<CodeDt> = None;
                    while let Some(keys) = mapp.next_key()? {
                        match keys.as_str() {
                            "id" => {
                                id = Some(mapp.next_value()?);
                            }
                            "extension" => {
                                extension = Some(mapp.next_value()?);
                            }
                            "modifierExtension" => {
                                modifier_extension = Some(mapp.next_value()?);
                            }
                            "other" => {
                                other = Some(mapp.next_value()?);
                            }
                            "type" => {
                                type_ = Some(mapp.next_value()?);
                            }
                            _ => {
                                return Err(
                                    FhirError::error_string({
                                        let res = ::alloc::fmt::format(
                                            format_args!("读到不存在的键:{0}", keys),
                                        );
                                        res
                                    }),
                                );
                            }
                        }
                    }
                    Ok(PatientLinkBackboneElement {
                        id,
                        extension,
                        modifier_extension,
                        other,
                        type_,
                    })
                }
            }
            deserializer.deserialize_struct("", PatientLinkBackboneElementVisitor)
        }
    }
    impl Executor for PatientLinkBackboneElement {
        fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
            match func.definition.function_name() {
                FunctionName::Element => {
                    match &func.params {
                        FunctionParam::String(name) => {
                            match name.as_str() {
                                "id" => self.id.exec(&func, paths),
                                "extension" => self.extension.exec(&func, paths),
                                "modifierExtension" => {
                                    self.modifier_extension.exec(&func, paths)
                                }
                                "other" => self.other.exec(&func, paths),
                                "type" => self.type_.exec(&func, paths),
                                other => {
                                    Err(
                                        FhirError::Message({
                                            let res = ::alloc::fmt::format(
                                                format_args!("无效的路径名:[{0}]", other),
                                            );
                                            res
                                        }),
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => {
                    Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!("Patient: 无效的函数名:{0:?}", &func),
                            );
                            res
                        }),
                    )
                }
            }
        }
        fn as_collection(&self) -> Collection {
            Collection(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Box::new(self.clone())]),
                ),
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientLinkBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientLinkBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "other",
                &self.other,
                "type_",
                &&self.type_,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientLinkBackboneElement {
        #[inline]
        fn clone(&self) -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                other: ::core::clone::Clone::clone(&self.other),
                type_: ::core::clone::Clone::clone(&self.type_),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientLinkBackboneElement {
        #[inline]
        fn default() -> PatientLinkBackboneElement {
            PatientLinkBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                other: ::core::default::Default::default(),
                type_: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientContactBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The kind of relationship
        #[fhir(
            name = "relationship",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub relationship: Option<Vec<CodeableConcept>>,
        /// A name associated with the contact person
        #[fhir(
            name = "name",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub name: Option<HumanName>,
        /// A contact detail for the person
        #[fhir(
            name = "telecom",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub telecom: Option<Vec<ContactPoint>>,
        /// Address for the contact person
        #[fhir(
            name = "address",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub address: Option<Address>,
        /// male | female | other | unknown
        #[fhir(
            name = "gender",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub gender: Option<CodeDt>,
        /// Organization that is associated with the contact
        #[fhir(
            name = "organization",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub organization: Option<Reference>,
        /// The period during which this contact person or organization is valid to be contacted relating to this patient
        #[fhir(
            name = "period",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub period: Option<Period>,
    }
    impl Base for PatientContactBackboneElement {}
    impl Serialize for PatientContactBackboneElement {
        fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
            let mut serialize_struct = serializer.serialize_struct("")?;
            serialize_struct.serialize_id(&self.id)?;
            serialize_struct.serialize_extension(&self.extension)?;
            serialize_struct
                .serialize_field("modifierExtension", &self.modifier_extension)?;
            serialize_struct.serialize_field("relationship", &self.relationship)?;
            serialize_struct.serialize_field("name", &self.name)?;
            serialize_struct.serialize_field("telecom", &self.telecom)?;
            serialize_struct.serialize_field("address", &self.address)?;
            serialize_struct.serialize_field("gender", &self.gender)?;
            serialize_struct.serialize_field("organization", &self.organization)?;
            serialize_struct.serialize_field("period", &self.period)?;
            serialize_struct.serialize_end()
        }
    }
    impl<'de> Deserialize<'de> for PatientContactBackboneElement {
        fn deserialize<De>(deserializer: De) -> Result<Self>
        where
            De: Deserializer<'de>,
        {
            pub struct PatientContactBackboneElementVisitor;
            impl<'de> Visitor<'de> for PatientContactBackboneElementVisitor {
                type Value = PatientContactBackboneElement;
                fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value>
                where
                    M: MapAccess<'de>,
                {
                    let mut id: Option<String> = None;
                    let mut extension: Option<Vec<Extension>> = None;
                    let mut modifier_extension: Option<Vec<Extension>> = None;
                    let mut relationship: Option<Vec<CodeableConcept>> = None;
                    let mut name: Option<HumanName> = None;
                    let mut telecom: Option<Vec<ContactPoint>> = None;
                    let mut address: Option<Address> = None;
                    let mut gender: Option<CodeDt> = None;
                    let mut organization: Option<Reference> = None;
                    let mut period: Option<Period> = None;
                    while let Some(keys) = mapp.next_key()? {
                        match keys.as_str() {
                            "id" => {
                                id = Some(mapp.next_value()?);
                            }
                            "extension" => {
                                extension = Some(mapp.next_value()?);
                            }
                            "modifierExtension" => {
                                modifier_extension = Some(mapp.next_value()?);
                            }
                            "relationship" => {
                                relationship = Some(mapp.next_value()?);
                            }
                            "name" => {
                                name = Some(mapp.next_value()?);
                            }
                            "telecom" => {
                                telecom = Some(mapp.next_value()?);
                            }
                            "address" => {
                                address = Some(mapp.next_value()?);
                            }
                            "gender" => {
                                gender = Some(mapp.next_value()?);
                            }
                            "organization" => {
                                organization = Some(mapp.next_value()?);
                            }
                            "period" => {
                                period = Some(mapp.next_value()?);
                            }
                            _ => {
                                return Err(
                                    FhirError::error_string({
                                        let res = ::alloc::fmt::format(
                                            format_args!("读到不存在的键:{0}", keys),
                                        );
                                        res
                                    }),
                                );
                            }
                        }
                    }
                    Ok(PatientContactBackboneElement {
                        id,
                        extension,
                        modifier_extension,
                        relationship,
                        name,
                        telecom,
                        address,
                        gender,
                        organization,
                        period,
                    })
                }
            }
            deserializer.deserialize_struct("", PatientContactBackboneElementVisitor)
        }
    }
    impl Executor for PatientContactBackboneElement {
        fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
            match func.definition.function_name() {
                FunctionName::Element => {
                    match &func.params {
                        FunctionParam::String(name) => {
                            match name.as_str() {
                                "id" => self.id.exec(&func, paths),
                                "extension" => self.extension.exec(&func, paths),
                                "modifierExtension" => {
                                    self.modifier_extension.exec(&func, paths)
                                }
                                "relationship" => self.relationship.exec(&func, paths),
                                "name" => self.name.exec(&func, paths),
                                "telecom" => self.telecom.exec(&func, paths),
                                "address" => self.address.exec(&func, paths),
                                "gender" => self.gender.exec(&func, paths),
                                "organization" => self.organization.exec(&func, paths),
                                "period" => self.period.exec(&func, paths),
                                other => {
                                    Err(
                                        FhirError::Message({
                                            let res = ::alloc::fmt::format(
                                                format_args!("无效的路径名:[{0}]", other),
                                            );
                                            res
                                        }),
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => {
                    Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!("Patient: 无效的函数名:{0:?}", &func),
                            );
                            res
                        }),
                    )
                }
            }
        }
        fn as_collection(&self) -> Collection {
            Collection(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Box::new(self.clone())]),
                ),
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientContactBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "extension",
                "modifier_extension",
                "relationship",
                "name",
                "telecom",
                "address",
                "gender",
                "organization",
                "period",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.extension,
                &self.modifier_extension,
                &self.relationship,
                &self.name,
                &self.telecom,
                &self.address,
                &self.gender,
                &self.organization,
                &&self.period,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "PatientContactBackboneElement",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientContactBackboneElement {
        #[inline]
        fn clone(&self) -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                relationship: ::core::clone::Clone::clone(&self.relationship),
                name: ::core::clone::Clone::clone(&self.name),
                telecom: ::core::clone::Clone::clone(&self.telecom),
                address: ::core::clone::Clone::clone(&self.address),
                gender: ::core::clone::Clone::clone(&self.gender),
                organization: ::core::clone::Clone::clone(&self.organization),
                period: ::core::clone::Clone::clone(&self.period),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientContactBackboneElement {
        #[inline]
        fn default() -> PatientContactBackboneElement {
            PatientContactBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                relationship: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                telecom: ::core::default::Default::default(),
                address: ::core::default::Default::default(),
                gender: ::core::default::Default::default(),
                organization: ::core::default::Default::default(),
                period: ::core::default::Default::default(),
            }
        }
    }
    pub struct PatientCommunicationBackboneElement {
        /// Unique id for inter-element referencing
        #[fhir(
            name = "id",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub id: Option<String>,
        /// Additional content defined by implementations
        #[fhir(
            name = "extension",
            min = "0",
            max = "*",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub extension: Option<Vec<Extension>>,
        /// Extensions that cannot be ignored even if unrecognized
        #[fhir(
            name = "modifierExtension",
            min = "0",
            max = "*",
            summary = true,
            modifier = true
        )]
        pub modifier_extension: Option<Vec<Extension>>,
        /// The language which can be used to communicate with the patient about his or her health
        #[fhir(
            name = "language",
            min = "1",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub language: Option<CodeableConcept>,
        /// Language preference indicator
        #[fhir(
            name = "preferred",
            min = "0",
            max = "1",
            summary = false,
            modifier = false,
            choice = ""
        )]
        pub preferred: Option<BooleanDt>,
    }
    impl Base for PatientCommunicationBackboneElement {}
    impl Serialize for PatientCommunicationBackboneElement {
        fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
            let mut serialize_struct = serializer.serialize_struct("")?;
            serialize_struct.serialize_id(&self.id)?;
            serialize_struct.serialize_extension(&self.extension)?;
            serialize_struct
                .serialize_field("modifierExtension", &self.modifier_extension)?;
            serialize_struct.serialize_field("language", &self.language)?;
            serialize_struct.serialize_field("preferred", &self.preferred)?;
            serialize_struct.serialize_end()
        }
    }
    impl<'de> Deserialize<'de> for PatientCommunicationBackboneElement {
        fn deserialize<De>(deserializer: De) -> Result<Self>
        where
            De: Deserializer<'de>,
        {
            pub struct PatientCommunicationBackboneElementVisitor;
            impl<'de> Visitor<'de> for PatientCommunicationBackboneElementVisitor {
                type Value = PatientCommunicationBackboneElement;
                fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value>
                where
                    M: MapAccess<'de>,
                {
                    let mut id: Option<String> = None;
                    let mut extension: Option<Vec<Extension>> = None;
                    let mut modifier_extension: Option<Vec<Extension>> = None;
                    let mut language: Option<CodeableConcept> = None;
                    let mut preferred: Option<BooleanDt> = None;
                    while let Some(keys) = mapp.next_key()? {
                        match keys.as_str() {
                            "id" => {
                                id = Some(mapp.next_value()?);
                            }
                            "extension" => {
                                extension = Some(mapp.next_value()?);
                            }
                            "modifierExtension" => {
                                modifier_extension = Some(mapp.next_value()?);
                            }
                            "language" => {
                                language = Some(mapp.next_value()?);
                            }
                            "preferred" => {
                                preferred = Some(mapp.next_value()?);
                            }
                            _ => {
                                return Err(
                                    FhirError::error_string({
                                        let res = ::alloc::fmt::format(
                                            format_args!("读到不存在的键:{0}", keys),
                                        );
                                        res
                                    }),
                                );
                            }
                        }
                    }
                    Ok(PatientCommunicationBackboneElement {
                        id,
                        extension,
                        modifier_extension,
                        language,
                        preferred,
                    })
                }
            }
            deserializer
                .deserialize_struct("", PatientCommunicationBackboneElementVisitor)
        }
    }
    impl Executor for PatientCommunicationBackboneElement {
        fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
            match func.definition.function_name() {
                FunctionName::Element => {
                    match &func.params {
                        FunctionParam::String(name) => {
                            match name.as_str() {
                                "id" => self.id.exec(&func, paths),
                                "extension" => self.extension.exec(&func, paths),
                                "modifierExtension" => {
                                    self.modifier_extension.exec(&func, paths)
                                }
                                "language" => self.language.exec(&func, paths),
                                "preferred" => self.preferred.exec(&func, paths),
                                other => {
                                    Err(
                                        FhirError::Message({
                                            let res = ::alloc::fmt::format(
                                                format_args!("无效的路径名:[{0}]", other),
                                            );
                                            res
                                        }),
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => {
                    Err(
                        FhirError::Message({
                            let res = ::alloc::fmt::format(
                                format_args!("Patient: 无效的函数名:{0:?}", &func),
                            );
                            res
                        }),
                    )
                }
            }
        }
        fn as_collection(&self) -> Collection {
            Collection(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Box::new(self.clone())]),
                ),
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatientCommunicationBackboneElement {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PatientCommunicationBackboneElement",
                "id",
                &self.id,
                "extension",
                &self.extension,
                "modifier_extension",
                &self.modifier_extension,
                "language",
                &self.language,
                "preferred",
                &&self.preferred,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatientCommunicationBackboneElement {
        #[inline]
        fn clone(&self) -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::clone::Clone::clone(&self.id),
                extension: ::core::clone::Clone::clone(&self.extension),
                modifier_extension: ::core::clone::Clone::clone(
                    &self.modifier_extension,
                ),
                language: ::core::clone::Clone::clone(&self.language),
                preferred: ::core::clone::Clone::clone(&self.preferred),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for PatientCommunicationBackboneElement {
        #[inline]
        fn default() -> PatientCommunicationBackboneElement {
            PatientCommunicationBackboneElement {
                id: ::core::default::Default::default(),
                extension: ::core::default::Default::default(),
                modifier_extension: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                preferred: ::core::default::Default::default(),
            }
        }
    }
}
