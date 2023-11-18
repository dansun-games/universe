use vk_parse as vk;

use super::var::VarDescriptor;
use crate::descriptors::alias::Alias;

pub fn get_commands(reg: &vk::Registry) -> (Vec<CommandDescriptor>, Vec<Alias>) {
	let mut filtered = reg.0.iter().filter_map(|item| match item {
		vk::RegistryChild::Commands(v) => Some(&v.children),
		_ => None,
	});

	let commands = filtered.next().expect("Could not find commands");
	assert_eq!(filtered.next(), None);

	let commands: Vec<_> = commands
		.iter()
		.filter_map(|v| match v {
			vk::Command::Definition(v) => Some(v),
			_ => None,
		})
		.map(|c| CommandDescriptor::from(c))
		.collect();

	//TODO: aliases
	let aliases: Vec<Alias> = vec![];

	(commands, aliases)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandDescriptor {
	pub name:           String,
	pub return_type:    String,
	pub params:         Vec<CommandParamDescriptor>,
	pub api_feat:       String,
	pub success_codes:  Vec<String>,
	pub error_codes:    Vec<String>,
	pub command_useage: CommandUseage,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandUseage {
	pub queue_types:        Vec<QueueType>,
	pub render_pass_scope:  Option<CommandIssueScope>,
	pub video_coding_scope: CommandIssueScope,
	pub buffer_levels:      CmdBufferLevels,
}

impl From<&vk::CommandDefinition> for CommandDescriptor {
	fn from(def: &vk::CommandDefinition) -> Self {
		assert_eq!(def.alias, None);
		// assert_eq!(def.videocoding, None);

		// println!("{:#?}", def.code);

		let api_feat = def.api.clone().unwrap_or(String::from("vulkan"));

		let queue_types: Vec<_> = def
			.queues
			.as_ref()
			.map(|q| q.split(',').map(QueueType::from).collect())
			.unwrap_or_default();

		let render_pass_scope = def
			.renderpass
			.as_ref()
			.map(String::as_str)
			.map(CommandIssueScope::from);

		let video_coding_scope = def
			.videocoding
			.as_ref()
			.map(String::as_str)
			.map(CommandIssueScope::from)
			.unwrap_or(CommandIssueScope::Outisde);

		let buffer_levels: CmdBufferLevels = def.cmdbufferlevel.as_ref().map(String::as_str).into();

		let error_codes: Vec<_> = def
			.errorcodes
			.as_ref()
			.map(|s| s.split(',').map(String::from).collect())
			.unwrap_or_default();

		let success_codes: Vec<_> = def
			.successcodes
			.as_ref()
			.map(|s| s.split(',').map(String::from).collect())
			.unwrap_or_default();

		let return_type = def.proto.type_name.clone().expect("Missing proto type");
		let name = def.proto.name.clone();
		let params: Vec<_> = def
			.params
			.iter()
			.map(CommandParamDescriptor::from)
			.collect();


		CommandDescriptor {
			name,
			return_type,
			params,
			api_feat,
			success_codes,
			error_codes,
			command_useage: CommandUseage {
				queue_types,
				render_pass_scope,
				video_coding_scope,
				buffer_levels,
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandParamDescriptor {
	pub var_spec:  VarDescriptor,
	pub api_feat:  Option<String>,
	pub sync_ctrl: bool,
}

impl From<&vk::CommandParam> for CommandParamDescriptor {
	fn from(def: &vk::CommandParam) -> Self {
		let var_spec = VarDescriptor::from(def);
		let api_feat = def.api.clone();
		//these are more than just "true", but in the case when they are, I think the sync info is captured in the struct
		//so we can extract the details from there.
		let sync_ctrl = def.externsync.as_ref().is_some();

		CommandParamDescriptor {
			var_spec,
			api_feat,
			sync_ctrl,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum QueueType {
	Graphics,
	Compute,
	Transfer,
	SparseBinding,
	Decode,
	Encode,
	OpticalFlow,
}

impl From<&str> for QueueType {
	fn from(value: &str) -> Self {
		match value {
			"graphics" => QueueType::Graphics,
			"compute" => QueueType::Compute,
			"transfer" => QueueType::Transfer,
			"sparse_binding" => QueueType::SparseBinding,
			"decode" => QueueType::Decode,
			"encode" => QueueType::Encode,
			"opticalflow" => QueueType::OpticalFlow,
			_ => panic!("Invalid Queue Type"),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct CmdBufferLevels {
	pub primary:   bool,
	pub secondary: bool,
}

impl From<Option<&str>> for CmdBufferLevels {
	fn from(value: Option<&str>) -> Self {
		let mut lvls = CmdBufferLevels {
			primary:   false,
			secondary: false,
		};

		if let Some(str) = value {
			for lvl in str.split(',') {
				match lvl {
					"primary" => lvls.primary = true,
					"secondary" => lvls.secondary = true,
					_ => panic!("Invalid Command Buffer Level"),
				}
			}
		}

		lvls
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandIssueScope {
	Inside,
	Outisde,
	Either,
}

impl From<&str> for CommandIssueScope {
	fn from(value: &str) -> Self {
		match value {
			"inside" => CommandIssueScope::Inside,
			"outside" => CommandIssueScope::Outisde,
			"both" => CommandIssueScope::Either,
			_ => panic!("Invalid Render Pass Timing"),
		}
	}
}
