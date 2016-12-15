//
// Copyright:: Copyright (c) 2016 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use cli::arguments::{pipeline_arg, config_path_arg, no_open_arg, project_arg,
          local_arg, config_project_arg, u_e_s_o_args, scp_args,
          value_of};
use clap::{App, SubCommand, ArgMatches};
use cli::InitCommand;
use config::Config;
use project;

pub const SUBCOMMAND_NAME: &'static str = "init";

#[derive(Debug)]
pub struct InitClapOptions<'n> {
    pub user: &'n str,
    pub server: &'n str,
    pub ent: &'n str,
    pub org: &'n str,
    pub project: &'n str,
    pub pipeline: &'n str,
    pub config_json: &'n str,
    pub generator: &'n str,
    pub github_org_name: &'n str,
    pub bitbucket_project_key: &'n str,
    pub repo_name: &'n str,
    pub no_v_ssl: bool,
    pub no_open: bool,
    pub skip_build_cookbook: bool,
    pub local: bool,
}
impl<'n> Default for InitClapOptions<'n> {
    fn default() -> Self {
        InitClapOptions {
            user: "",
            server: "",
            ent: "",
            org: "",
            project: "",
            pipeline: "master",
            config_json: "",
            generator: "",
            github_org_name: "",
            bitbucket_project_key: "",
            repo_name: "",
            no_v_ssl: false,
            no_open: false,
            skip_build_cookbook: false,
            local: false,
        }
    }
}

impl<'n> InitClapOptions<'n> {
    pub fn new(matches: &'n ArgMatches<'n>) -> Self {
        InitClapOptions {
            user: value_of(&matches, "user"),
            server: value_of(&matches, "server"),
            ent: value_of(&matches, "ent"),
            org: value_of(&matches, "org"),
            project: value_of(&matches, "project"),
            pipeline: value_of(&matches, "pipeline"),
            config_json: value_of(&matches, "config-json"),
            generator: value_of(&matches, "generator"),
            github_org_name: value_of(&matches, "github"),
            bitbucket_project_key: value_of(&matches, "bitbucket"),
            repo_name: value_of(&matches, "repo-name"),
            no_v_ssl: matches.is_present("no-verify-ssl"),
            no_open: matches.is_present("no-open"),
            skip_build_cookbook: matches.is_present("skip-build-cookbook"),
            local: matches.is_present("local"),
        }
    }
}

impl<'n> InitCommand for InitClapOptions<'n> {
    fn merge_options_and_config(&self, config: Config) -> Config {
        let final_proj = project::project_or_from_cwd(&self.project).unwrap();

        let new_config = config.set_user(&self.user)
            .set_server(&self.server)
            .set_enterprise(&self.ent)
            .set_organization(&self.org)
            .set_project(&final_proj)
            .set_pipeline(&self.pipeline)
            .set_generator(&self.generator)
            .set_config_json(&self.config_json);
        return new_config;
    }
}

pub fn clap_subcommand<'c>() -> App<'c, 'c> {
    SubCommand::with_name(SUBCOMMAND_NAME)
        .about("Initialize a Delivery project \
                (and lots more!)")
        .args(&vec![config_path_arg(), no_open_arg(), project_arg(),
                local_arg(), config_project_arg()])
        .args_from_usage(
            "--generator=[generator] 'Local path or Git repo URL to a \
             custom ChefDK build_cookbook generator (default:github)'
            --skip-build-cookbook 'Do not create a build cookbook'")
        .args(&u_e_s_o_args())
        .args(&scp_args())
        .args(&pipeline_arg())
}
