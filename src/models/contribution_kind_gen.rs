use clap::ValueEnum;
use color_eyre::eyre::{Error, Result, eyre};
use std::fmt;

#[derive(Debug, Clone, ValueEnum)]
pub enum ContributionKind {
	Audio,
	A11Y,
	Bug,
	Blog,
	Business,
	Code,
	Content,
	Data,
	Doc,
	Design,
	Example,
	EventOrganizing,
	Financial,
	FundingFinding,
	Ideas,
	Infra,
	Maintenance,
	Mentoring,
	Platform,
	Plugin,
	ProjectManagement,
	Promotion,
	Question,
	Research,
	Review,
	Security,
	Tool,
	Translation,
	Test,
	Tutorial,
	Talk,
	UserTesting,
	Video,
}

impl TryFrom<&str> for ContributionKind {
	type Error = Error;

	fn try_from(code: &str) -> Result<Self> {
		match code {
			"audio" => Ok(ContributionKind::Audio),
			"a11y" => Ok(ContributionKind::A11Y),
			"bug" => Ok(ContributionKind::Bug),
			"blog" => Ok(ContributionKind::Blog),
			"business" => Ok(ContributionKind::Business),
			"code" => Ok(ContributionKind::Code),
			"content" => Ok(ContributionKind::Content),
			"data" => Ok(ContributionKind::Data),
			"doc" => Ok(ContributionKind::Doc),
			"design" => Ok(ContributionKind::Design),
			"example" => Ok(ContributionKind::Example),
			"eventOrganizing" => Ok(ContributionKind::EventOrganizing),
			"financial" => Ok(ContributionKind::Financial),
			"fundingFinding" => Ok(ContributionKind::FundingFinding),
			"ideas" => Ok(ContributionKind::Ideas),
			"infra" => Ok(ContributionKind::Infra),
			"maintenance" => Ok(ContributionKind::Maintenance),
			"mentoring" => Ok(ContributionKind::Mentoring),
			"platform" => Ok(ContributionKind::Platform),
			"plugin" => Ok(ContributionKind::Plugin),
			"projectManagement" => Ok(ContributionKind::ProjectManagement),
			"promotion" => Ok(ContributionKind::Promotion),
			"question" => Ok(ContributionKind::Question),
			"research" => Ok(ContributionKind::Research),
			"review" => Ok(ContributionKind::Review),
			"security" => Ok(ContributionKind::Security),
			"tool" => Ok(ContributionKind::Tool),
			"translation" => Ok(ContributionKind::Translation),
			"test" => Ok(ContributionKind::Test),
			"tutorial" => Ok(ContributionKind::Tutorial),
			"talk" => Ok(ContributionKind::Talk),
			"userTesting" => Ok(ContributionKind::UserTesting),
			"video" => Ok(ContributionKind::Video),
			_ => Err(eyre!("The contribution type '{}' does not exist.", code))
		}
	}
}

impl fmt::Display for ContributionKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ContributionKind::Audio => write!(f, "audio"),
			ContributionKind::A11Y => write!(f, "a11y"),
			ContributionKind::Bug => write!(f, "bug"),
			ContributionKind::Blog => write!(f, "blog"),
			ContributionKind::Business => write!(f, "business"),
			ContributionKind::Code => write!(f, "code"),
			ContributionKind::Content => write!(f, "content"),
			ContributionKind::Data => write!(f, "data"),
			ContributionKind::Doc => write!(f, "doc"),
			ContributionKind::Design => write!(f, "design"),
			ContributionKind::Example => write!(f, "example"),
			ContributionKind::EventOrganizing => write!(f, "eventOrganizing"),
			ContributionKind::Financial => write!(f, "financial"),
			ContributionKind::FundingFinding => write!(f, "fundingFinding"),
			ContributionKind::Ideas => write!(f, "ideas"),
			ContributionKind::Infra => write!(f, "infra"),
			ContributionKind::Maintenance => write!(f, "maintenance"),
			ContributionKind::Mentoring => write!(f, "mentoring"),
			ContributionKind::Platform => write!(f, "platform"),
			ContributionKind::Plugin => write!(f, "plugin"),
			ContributionKind::ProjectManagement => write!(f, "projectManagement"),
			ContributionKind::Promotion => write!(f, "promotion"),
			ContributionKind::Question => write!(f, "question"),
			ContributionKind::Research => write!(f, "research"),
			ContributionKind::Review => write!(f, "review"),
			ContributionKind::Security => write!(f, "security"),
			ContributionKind::Tool => write!(f, "tool"),
			ContributionKind::Translation => write!(f, "translation"),
			ContributionKind::Test => write!(f, "test"),
			ContributionKind::Tutorial => write!(f, "tutorial"),
			ContributionKind::Talk => write!(f, "talk"),
			ContributionKind::UserTesting => write!(f, "userTesting"),
			ContributionKind::Video => write!(f, "video"),
		}
	}
}
