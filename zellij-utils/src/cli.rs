use crate::data::{Direction, InputMode, Resize, UnblockCondition};
use crate::setup::Setup;
use crate::{
    consts::{ZELLIJ_CONFIG_DIR_ENV, ZELLIJ_CONFIG_FILE_ENV},
    input::{layout::PluginUserConfiguration, options::Options},
};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;
use url::Url;

fn validate_session(name: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        use crate::consts::ZELLIJ_SOCK_MAX_LENGTH;

        let mut socket_path = crate::consts::ZELLIJ_SOCK_DIR.clone();
        socket_path.push(name);

        if socket_path.as_os_str().len() >= ZELLIJ_SOCK_MAX_LENGTH {
            // socket path must be less than 108 bytes
            let available_length = ZELLIJ_SOCK_MAX_LENGTH
                .saturating_sub(socket_path.as_os_str().len())
                .saturating_sub(1);

            return Err(format!(
                "会话名称长度必须小于 {} 个字符",
                available_length
            ));
        };
    };

    Ok(name.to_owned())
}

#[derive(Parser, Default, Debug, Clone, Serialize, Deserialize)]
#[clap(version, name = "zellij")]
pub struct CliArgs {
    /// 屏幕上的最大窗格数，注意：超过后新开窗格会关闭旧窗格
    #[clap(long, value_parser)]
    pub max_panes: Option<usize>,

    /// 更改 zellij 查找插件的位置
    #[clap(long, value_parser, overrides_with = "data_dir")]
    pub data_dir: Option<PathBuf>,

    /// 在指定 socket 路径监听并运行服务端
    #[clap(long, value_parser, hide = true, overrides_with = "server")]
    pub server: Option<PathBuf>,

    /// 指定新会话名称
    #[clap(long, short, overrides_with = "session", value_parser = validate_session)]
    pub session: Option<String>,

    /// 布局目录中的预设布局名，或布局文件路径
    /// 若当前已在会话内（或使用 --session），会作为新标签页加入该会话
    /// 否则将启动新会话
    #[clap(short, long, value_parser, overrides_with = "layout")]
    pub layout: Option<PathBuf>,

    /// 布局目录中的预设布局名，或布局文件路径
    /// 始终启动新会话，即使当前已在会话中
    #[clap(short, long, value_parser, overrides_with = "new_session_with_layout")]
    pub new_session_with_layout: Option<PathBuf>,

    /// 更改 zellij 查找配置文件的位置
    #[clap(short, long, overrides_with = "config", env = ZELLIJ_CONFIG_FILE_ENV, value_parser)]
    pub config: Option<PathBuf>,

    /// 更改 zellij 查找配置目录的位置
    #[clap(long, overrides_with = "config_dir", env = ZELLIJ_CONFIG_DIR_ENV, value_parser)]
    pub config_dir: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Command>,

    /// 输出额外调试信息
    #[clap(short, long, value_parser)]
    pub debug: bool,
}

impl CliArgs {
    pub fn is_setup_clean(&self) -> bool {
        if let Some(Command::Setup(ref setup)) = &self.command {
            if setup.clean {
                return true;
            }
        }
        false
    }
    pub fn options(&self) -> Option<Options> {
        if let Some(Command::Options(options)) = &self.command {
            return Some(options.clone());
        }
        None
    }
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum Command {
    /// 修改 zellij 行为
    #[clap(name = "options", value_parser)]
    Options(Options),

    /// 设置 zellij 并检查配置
    #[clap(name = "setup", value_parser)]
    Setup(Setup),

    /// 运行用于提供终端会话的 Web 服务
    #[clap(name = "web", value_parser)]
    Web(WebCli),

    /// 浏览现有 zellij 会话
    #[clap(flatten)]
    Sessions(Sessions),
}

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub struct WebCli {
    /// 启动服务（若未指定其他参数则为默认行为）
    #[clap(long, value_parser, display_order = 1)]
    pub start: bool,

    /// 停止服务
    #[clap(long, value_parser, exclusive(true), display_order = 2)]
    pub stop: bool,

    /// 获取服务状态
    #[clap(long, value_parser, conflicts_with("start"), display_order = 3)]
    pub status: bool,

    /// 状态检查超时时间（秒，默认：30）
    #[clap(long, value_parser, requires = "status", display_order = 4)]
    pub timeout: Option<u64>,

    /// 后台运行服务
    #[clap(
        short,
        long,
        value_parser,
        conflicts_with_all(&["stop", "status", "create-token", "revoke-token", "revoke-all-tokens"]),
        display_order = 5
    )]
    pub daemonize: bool,
    /// 为 Web 界面创建登录令牌，只会显示一次，后续无法再取回
    /// 返回令牌名称和令牌值
    #[clap(long, value_parser, exclusive(true), display_order = 6)]
    pub create_token: bool,
    /// 令牌可选名称
    #[clap(long, value_parser, value_name = "TOKEN_NAME", display_order = 7)]
    pub token_name: Option<String>,
    /// 创建只读登录令牌（仅可作为观察者连接现有会话）
    #[clap(long, value_parser, exclusive(true), display_order = 8)]
    pub create_read_only_token: bool,
    /// 按名称吊销登录令牌
    #[clap(
        long,
        value_parser,
        exclusive(true),
        value_name = "TOKEN NAME",
        display_order = 9
    )]
    pub revoke_token: Option<String>,
    /// 吊销全部登录令牌
    #[clap(long, value_parser, exclusive(true), display_order = 10)]
    pub revoke_all_tokens: bool,
    /// 列出令牌名称及创建时间（不会显示令牌内容）
    #[clap(long, value_parser, exclusive(true), display_order = 11)]
    pub list_tokens: bool,
    /// 本地监听 IP 地址（默认 127.0.0.1）
    #[clap(
        long,
        value_parser,
        conflicts_with_all(&["stop", "status", "create-token", "revoke-token", "revoke-all-tokens"]),
        display_order = 12
    )]
    pub ip: Option<IpAddr>,
    /// 本地监听端口（默认 8082）
    #[clap(
        long,
        value_parser,
        conflicts_with_all(&["stop", "status", "create-token", "revoke-token", "revoke-all-tokens"]),
        display_order = 13
    )]
    pub port: Option<u16>,
    /// SSL 证书路径（若监听地址不是 127.0.0.1 则必填）
    #[clap(
        long,
        value_parser,
        conflicts_with_all(&["stop", "status", "create-token", "revoke-token", "revoke-all-tokens"]),
        display_order = 14
    )]
    pub cert: Option<PathBuf>,
    /// SSL 私钥路径（若监听地址不是 127.0.0.1 则必填）
    #[clap(
        long,
        value_parser,
        conflicts_with_all(&["stop", "status", "create-token", "revoke-token", "revoke-all-tokens"]),
        display_order = 15
    )]
    pub key: Option<PathBuf>,
}

impl WebCli {
    pub fn get_start(&self) -> bool {
        self.start
            || !(self.stop
                || self.status
                || self.create_token
                || self.create_read_only_token
                || self.revoke_token.is_some()
                || self.revoke_all_tokens
                || self.list_tokens)
    }
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum SessionCommand {
    /// 修改 zellij 行为
    #[clap(name = "options")]
    Options(Options),
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum Sessions {
    /// 列出活动会话
    #[clap(visible_alias = "ls")]
    ListSessions {
        /// 列表不添加颜色和格式（便于解析）
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        no_formatting: bool,

        /// 仅输出会话名称
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        short: bool,

        /// 反向列出会话（默认升序）
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        reverse: bool,
    },
    /// 列出现有插件别名
    #[clap(visible_alias = "la")]
    ListAliases,
    /// 连接到会话
    #[clap(visible_alias = "a")]
    Attach {
        /// 要连接的会话名称
        #[clap(value_parser)]
        session_name: Option<String>,

        /// 若会话不存在则创建
        #[clap(short, long, value_parser)]
        create: bool,

        /// 若会话不存在则在后台创建分离会话
        #[clap(short('b'), long, value_parser)]
        create_background: bool,

        /// 按创建时间排序的活动会话索引编号
        #[clap(long, value_parser)]
        index: Option<usize>,

        /// 修改 zellij 行为
        #[clap(subcommand, name = "options")]
        options: Option<Box<SessionCommand>>,

        /// 若恢复已退出会话，启动后立即运行其全部命令
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        force_run_commands: bool,

        /// 远程会话认证令牌
        #[clap(short('t'), long, value_parser)]
        token: Option<String>,

        /// 保存会话用于自动重新认证（4 周）
        #[clap(short('r'), long, value_parser)]
        remember: bool,

        /// 连接前删除已保存会话
        #[clap(long, value_parser)]
        forget: bool,
    },

    /// 观察会话（只读）
    #[clap(visible_alias = "w")]
    Watch {
        /// 要观察的会话名称
        #[clap(value_parser)]
        session_name: Option<String>,
    },

    /// 终止指定会话
    #[clap(visible_alias = "k")]
    KillSession {
        /// 目标会话名称
        #[clap(value_parser)]
        target_session: Option<String>,
    },

    /// 删除指定会话
    #[clap(visible_alias = "d")]
    DeleteSession {
        /// 目标会话名称
        #[clap(value_parser)]
        target_session: Option<String>,
        /// 删除前若会话仍在运行则先终止
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        force: bool,
    },

    /// 终止所有会话
    #[clap(visible_alias = "ka")]
    KillAllSessions {
        /// 对提示自动回答 yes
        #[clap(short, long, value_parser)]
        yes: bool,
    },

    /// 删除所有会话
    #[clap(visible_alias = "da")]
    DeleteAllSessions {
        /// 对提示自动回答 yes
        #[clap(short, long, value_parser)]
        yes: bool,
        /// 删除前若会话仍在运行则先终止
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        force: bool,
    },

    /// 向指定会话发送动作
    #[clap(visible_alias = "ac")]
    #[clap(subcommand)]
    Action(CliAction),
    /// 在新窗格中运行命令
    /// 返回：创建的窗格 ID（格式：terminal_<id>）
    #[clap(visible_alias = "r")]
    Run {
        /// 要运行的命令
        #[clap(last(true), required(true))]
        command: Vec<String>,

        /// 新窗格打开方向
        #[clap(short, long, value_parser, conflicts_with("floating"))]
        direction: Option<Direction>,

        /// 更改新窗格工作目录
        #[clap(long, value_parser)]
        cwd: Option<PathBuf>,

        /// 以浮动模式打开新窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        floating: bool,

        /// 在当前窗格位置打开新窗格，并临时挂起当前窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("floating"),
            conflicts_with("direction")
        )]
        in_place: bool,

        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,

        /// 新窗格名称
        #[clap(short, long, value_parser)]
        name: Option<String>,

        /// 命令退出后立即关闭窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        close_on_exit: bool,

        /// 以挂起状态启动命令，首次按下 ENTER 后才运行
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        start_suspended: bool,

        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long, requires("floating"))]
        pinned: Option<bool>,
        #[clap(
            long,
            conflicts_with("floating"),
            conflicts_with("direction"),
            value_parser,
            default_value("false"),
            takes_value(false)
        )]
        stacked: bool,
        /// 阻塞直到命令结束且窗格已关闭
        #[clap(long, value_parser, default_value("false"), takes_value(false))]
        blocking: bool,

        /// 阻塞直到命令成功退出（状态码 0）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("blocking"),
            conflicts_with("block-until-exit-failure"),
            conflicts_with("block-until-exit")
        )]
        block_until_exit_success: bool,

        /// 阻塞直到命令失败退出（非 0 状态码）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("blocking"),
            conflicts_with("block-until-exit-success"),
            conflicts_with("block-until-exit")
        )]
        block_until_exit_failure: bool,

        /// 阻塞直到命令退出（无论状态码）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("blocking"),
            conflicts_with("block-until-exit-success"),
            conflicts_with("block-until-exit-failure")
        )]
        block_until_exit: bool,
        /// 若设置，将在当前窗格附近打开，而非跟随用户焦点
        #[clap(long)]
        near_current_pane: bool,
        /// 无边框启动该窗格（警告：将无法通过鼠标移动）
        #[clap(short, long, value_parser)]
        borderless: Option<bool>,
    },
    /// 加载插件
    /// 返回：创建的窗格 ID（格式：plugin_<id>）
    #[clap(visible_alias = "p")]
    Plugin {
        /// 插件 URL，可为 http(s)、file: 或 zellij: 开头
        #[clap(last(true), required(true))]
        url: String,

        /// 插件配置
        #[clap(short, long, value_parser)]
        configuration: Option<PluginUserConfiguration>,

        /// 以浮动模式打开新窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        floating: bool,

        /// 在当前窗格位置打开新窗格，并临时挂起当前窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("floating")
        )]
        in_place: bool,

        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,

        /// 跳过内存和磁盘缓存并强制重新编译插件（适合开发场景）
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        skip_plugin_cache: bool,
        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long, requires("floating"))]
        pinned: Option<bool>,
        /// 无边框启动该窗格（警告：将无法通过鼠标移动）
        #[clap(short, long, value_parser)]
        borderless: Option<bool>,
    },
    /// 使用默认 $EDITOR / $VISUAL 编辑文件
    /// 返回：创建的窗格 ID（格式：terminal_<id>）
    #[clap(visible_alias = "e")]
    Edit {
        file: PathBuf,

        /// 在指定行号打开文件
        #[clap(short, long, value_parser)]
        line_number: Option<usize>,

        /// 新窗格打开方向
        #[clap(short, long, value_parser, conflicts_with("floating"))]
        direction: Option<Direction>,

        /// 在当前窗格位置打开新窗格，并临时挂起当前窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("floating"),
            conflicts_with("direction")
        )]
        in_place: bool,

        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,

        /// 以浮动模式打开新窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        floating: bool,

        /// 更改编辑器工作目录
        #[clap(long, value_parser)]
        cwd: Option<PathBuf>,
        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long, requires("floating"))]
        pinned: Option<bool>,
        /// 若设置，将在当前窗格附近打开，而非跟随用户焦点
        #[clap(long)]
        near_current_pane: bool,
        /// 无边框启动该窗格（警告：将无法通过鼠标移动）
        #[clap(short, long, value_parser)]
        borderless: Option<bool>,
    },
    ConvertConfig {
        old_config_file: PathBuf,
    },
    ConvertLayout {
        old_layout_file: PathBuf,
    },
    ConvertTheme {
        old_theme_file: PathBuf,
    },
    /// 向一个或多个插件发送数据，若插件未运行则启动
    #[clap(override_usage(
r#"
zellij pipe [OPTIONS] [--] <PAYLOAD>

* 向指定插件发送数据：

zellij pipe --plugin file:/path/to/my/plugin.wasm --name my_pipe_name -- my_arbitrary_data

* 发送到所有正在监听的运行中插件：

zellij pipe --name my_pipe_name -- my_arbitrary_data

* 将数据管道到此命令的 STDIN，并从插件输出到此命令的 STDOUT

tail -f /tmp/my-live-logfile | zellij pipe --name logs --plugin https://example.com/my-plugin.wasm | wc -l
"#))]
    Pipe {
        /// 管道名称
        #[clap(short, long, value_parser, display_order(1))]
        name: Option<String>,
        /// 通过此管道发送的数据（为空则监听 STDIN）
        payload: Option<String>,

        #[clap(short, long, value_parser, display_order(2))]
        /// 管道参数
        args: Option<PluginUserConfiguration>, // TODO: we might want to not re-use
        // PluginUserConfiguration
        /// 此管道目标插件 URL（如 file:/tmp/my-plugin.wasm）
        /// 若不指定则发送到所有插件；若指定且未运行则启动插件
        #[clap(short, long, value_parser, display_order(3))]
        plugin: Option<String>,
        /// 插件配置（注意：同一插件不同配置会被视为不同插件以决定管道目标）
        #[clap(short('c'), long, value_parser, display_order(4))]
        plugin_configuration: Option<PluginUserConfiguration>,
    },
    /// 向 sequence 插件发送命令（等价于：zellij pipe --plugin zellij:sequence）
    #[clap(
        visible_alias = "seq",
        override_usage(
            r#"
zellij sequence [OPTIONS] [--] <COMMANDS>

* 运行一组命令序列：

zellij sequence -- 'echo hello && echo world'

* 从 STDIN 管道输入命令：

echo 'echo hello && echo world' | zellij sequence

* 在返回前等待序列执行完成：

zellij sequence --blocking -- 'echo hello && echo world'
"#
        )
    )]
    Sequence {
        /// 要运行的命令（为空则监听 STDIN）
        payload: Option<String>,
        /// 退出前阻塞直到序列执行完成
        #[clap(short, long, value_parser, takes_value(false), default_value("false"))]
        blocking: bool,
    },
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum CliAction {
    /// 向终端写入字节
    Write {
        bytes: Vec<u8>,
        /// 目标 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: Option<String>,
    },
    /// 向终端写入字符
    WriteChars {
        chars: String,
        /// 目标 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: Option<String>,
    },
    /// 向终端发送一个或多个按键（如 "Ctrl a"、"F1"、"Alt Shift b"）
    SendKeys {
        /// 要发送的按键（空格分隔字符串）
        #[clap(value_parser, required = true)]
        keys: Vec<String>,

        /// 目标 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: Option<String>,
    },
    /// 在聚焦窗格的 [left|down|up|right] 边执行 [increase|decrease] 调整
    Resize {
        resize: Resize,
        direction: Option<Direction>,
    },
    /// 焦点切换到下一个窗格
    FocusNextPane,
    /// 焦点切换到上一个窗格
    FocusPreviousPane,
    /// 将聚焦窗格移动到指定方向 [right|left|up|down]
    MoveFocus {
        direction: Direction,
    },
    /// 按指定方向移动焦点到窗格或标签页（若在屏幕边缘）
    /// [right|left|up|down]
    MoveFocusOrTab {
        direction: Direction,
    },
    /// 按指定方向改变聚焦窗格位置，或向前轮换
    /// [right|left|up|down]
    MovePane {
        direction: Option<Direction>,
    },
    /// 将前一个窗格位置向后轮换
    MovePaneBackwards,
    /// 清空聚焦窗格所有缓冲区
    Clear,
    /// 将聚焦窗格内容导出到文件
    DumpScreen {
        path: PathBuf,

        /// 导出窗格完整回滚缓冲区
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        full: bool,
    },
    /// 将当前布局输出到 stdout
    DumpLayout,
    /// 立即将当前会话状态保存到磁盘
    SaveSession,
    /// 在默认编辑器中打开窗格回滚缓冲区
    EditScrollback,
    /// 在聚焦窗格向上滚动
    ScrollUp,
    /// 在聚焦窗格向下滚动
    ScrollDown,
    /// 在聚焦窗格滚动到底部
    ScrollToBottom,
    /// 在聚焦窗格滚动到顶部
    ScrollToTop,
    /// 在聚焦窗格向上翻一页
    PageScrollUp,
    /// 在聚焦窗格向下翻一页
    PageScrollDown,
    /// 在聚焦窗格向上翻半页
    HalfPageScrollUp,
    /// 在聚焦窗格向下翻半页
    HalfPageScrollDown,
    /// 在聚焦窗格全屏与普通布局间切换
    ToggleFullscreen,
    /// 切换 UI 中窗格边框显示
    TogglePaneFrames,
    /// 在“向当前标签页全部窗格发送文本命令”与普通模式间切换
    ToggleActiveSyncTab,
    /// 在指定方向 [right|down] 打开新窗格
    /// 若未指定方向，将尝试使用可用最大空间
    /// 返回：创建的窗格 ID（格式：terminal_<id> 或 plugin_<id>）
    NewPane {
        /// 新窗格打开方向
        #[clap(short, long, value_parser, conflicts_with("floating"))]
        direction: Option<Direction>,

        #[clap(last(true))]
        command: Vec<String>,

        #[clap(short, long, conflicts_with("command"), conflicts_with("direction"))]
        plugin: Option<String>,

        /// 更改新窗格工作目录
        #[clap(long, value_parser)]
        cwd: Option<PathBuf>,

        /// 以浮动模式打开新窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        floating: bool,

        /// 在当前窗格位置打开新窗格，并临时挂起当前窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("floating"),
            conflicts_with("direction")
        )]
        in_place: bool,

        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,

        /// 新窗格名称
        #[clap(short, long, value_parser)]
        name: Option<String>,

        /// 命令退出后立即关闭窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("command")
        )]
        close_on_exit: bool,
        /// 以挂起状态启动命令，首次按下 ENTER 后才运行
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("command")
        )]
        start_suspended: bool,
        #[clap(long, value_parser)]
        configuration: Option<PluginUserConfiguration>,
        #[clap(long, value_parser)]
        skip_plugin_cache: bool,
        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long, requires("floating"))]
        pinned: Option<bool>,
        #[clap(
            long,
            conflicts_with("floating"),
            conflicts_with("direction"),
            value_parser,
            default_value("false"),
            takes_value(false)
        )]
        stacked: bool,
        #[clap(short, long)]
        blocking: bool,

        // TODO: clean this up
        #[clap(skip)]
        unblock_condition: Option<UnblockCondition>,

        /// 若设置，将在当前窗格附近打开，而非跟随用户焦点
        #[clap(long)]
        near_current_pane: bool,
        /// 无边框启动该窗格（警告：将无法通过鼠标移动）
        #[clap(long, value_parser)]
        borderless: Option<bool>,
    },
    /// 在新 zellij 窗格中用默认 EDITOR 打开指定文件
    /// 返回：创建的窗格 ID（格式：terminal_<id>）
    Edit {
        file: PathBuf,

        /// 新窗格打开方向
        #[clap(short, long, value_parser, conflicts_with("floating"))]
        direction: Option<Direction>,

        /// 在指定行号打开文件
        #[clap(short, long, value_parser)]
        line_number: Option<usize>,

        /// 以浮动模式打开新窗格
        #[clap(short, long, value_parser, default_value("false"), takes_value(false))]
        floating: bool,

        /// 在当前窗格位置打开新窗格，并临时挂起当前窗格
        #[clap(
            short,
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            conflicts_with("floating"),
            conflicts_with("direction")
        )]
        in_place: bool,

        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,

        /// 更改编辑器工作目录
        #[clap(long, value_parser)]
        cwd: Option<PathBuf>,
        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long, requires("floating"))]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long, requires("floating"))]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long, requires("floating"))]
        pinned: Option<bool>,
        /// 若设置，将在当前窗格附近打开，而非跟随用户焦点
        #[clap(long)]
        near_current_pane: bool,
        /// 无边框启动该窗格（警告：将无法通过鼠标移动）
        #[clap(short, long, value_parser)]
        borderless: Option<bool>,
    },
    /// 切换所有已连接客户端的输入模式 [locked|pane|tab|resize|move|search|session]
    SwitchMode {
        input_mode: InputMode,
    },
    /// 若聚焦窗格为浮动则内嵌，若为内嵌则浮动
    TogglePaneEmbedOrFloating,
    /// 切换当前标签页所有浮动窗格可见性；若不存在则打开一个
    ToggleFloatingPanes,
    /// 关闭聚焦窗格
    ClosePane,
    /// 重命名聚焦窗格
    RenamePane {
        name: String,
    },
    /// 移除先前设置的窗格名称
    UndoRenamePane,
    /// 跳转到下一个标签页
    GoToNextTab,
    /// 跳转到上一个标签页
    GoToPreviousTab,
    /// 关闭当前标签页
    CloseTab,
    /// 跳转到索引为 [index] 的标签页
    GoToTab {
        index: u32,
    },
    /// 跳转到名称为 [name] 的标签页
    ///
    /// 返回：使用 --create 且创建成功时，输出单个数字的 tab ID
    GoToTabName {
        name: String,
        /// 若标签页不存在则创建
        #[clap(short, long, value_parser)]
        create: bool,
    },
    /// 重命名聚焦标签页
    RenameTab {
        name: String,
    },
    /// 移除先前设置的标签页名称
    UndoRenameTab,
    /// 跳转到指定稳定 ID 的标签页
    GoToTabById {
        id: u64,
    },
    /// 关闭指定稳定 ID 的标签页
    CloseTabById {
        id: u64,
    },
    /// 按稳定 ID 重命名标签页
    RenameTabById {
        id: u64,
        name: String,
    },
    /// 创建新标签页，可选指定布局和名称
    ///
    /// 返回：在 stdout 输出单个数字的已创建 tab ID
    NewTab {
        /// 新标签页使用的布局
        #[clap(short, long, value_parser)]
        layout: Option<PathBuf>,

        /// 查找布局的默认目录
        #[clap(long, value_parser, requires("layout"))]
        layout_dir: Option<PathBuf>,

        /// 新标签页名称
        #[clap(short, long, value_parser)]
        name: Option<String>,

        /// 更改新标签页工作目录
        #[clap(short, long, value_parser)]
        cwd: Option<PathBuf>,

        /// 新标签页可选初始命令
        #[clap(
            value_parser,
            conflicts_with("initial-plugin"),
            multiple_values(true),
            takes_value(true),
            last(true)
        )]
        initial_command: Vec<String>,

        /// 在新标签页中加载的初始插件
        #[clap(long, value_parser, conflicts_with("initial-command"))]
        initial_plugin: Option<String>,

        /// 命令退出后立即关闭窗格
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("initial-command")
        )]
        close_on_exit: bool,

        /// 以挂起状态启动命令，首次按下 ENTER 后才运行
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("initial-command")
        )]
        start_suspended: bool,

        /// 阻塞直到命令成功退出（状态码 0）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("initial-command"),
            conflicts_with("block-until-exit-failure"),
            conflicts_with("block-until-exit")
        )]
        block_until_exit_success: bool,

        /// 阻塞直到命令失败退出（非 0 状态码）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("initial-command"),
            conflicts_with("block-until-exit-success"),
            conflicts_with("block-until-exit")
        )]
        block_until_exit_failure: bool,

        /// 阻塞直到命令退出（无论状态码）或窗格已关闭
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("initial-command"),
            conflicts_with("block-until-exit-success"),
            conflicts_with("block-until-exit-failure")
        )]
        block_until_exit: bool,
    },
    /// 按指定方向移动聚焦标签页 [right|left]
    MoveTab {
        direction: Direction,
    },
    PreviousSwapLayout,
    NextSwapLayout,
    /// 覆盖当前活动标签页布局
    OverrideLayout {
        /// 布局文件路径
        #[clap(value_parser)]
        layout: PathBuf,

        /// 查找布局的默认目录
        #[clap(long, value_parser)]
        layout_dir: Option<PathBuf>,

        /// 保留布局中放不下的现有终端窗格（默认：false）
        #[clap(long, value_parser, takes_value(false), default_value("false"))]
        retain_existing_terminal_panes: bool,

        /// 保留布局中放不下的现有插件窗格（默认：false）
        #[clap(long, value_parser, takes_value(false), default_value("false"))]
        retain_existing_plugin_panes: bool,

        /// 仅对活动标签页应用布局（若布局含多个 tab，仅使用第一个）
        #[clap(long, value_parser, takes_value(false), default_value("false"))]
        apply_only_to_active_tab: bool,
    },
    /// 查询所有标签页名称
    QueryTabNames,
    StartOrReloadPlugin {
        url: String,
        #[clap(short, long, value_parser)]
        configuration: Option<PluginUserConfiguration>,
    },
    /// 返回：创建或聚焦插件时的插件窗格 ID（格式：plugin_<id>）
    LaunchOrFocusPlugin {
        #[clap(short, long, value_parser)]
        floating: bool,
        #[clap(short, long, value_parser)]
        in_place: bool,
        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,
        #[clap(short, long, value_parser)]
        move_to_focused_tab: bool,
        url: String,
        #[clap(short, long, value_parser)]
        configuration: Option<PluginUserConfiguration>,
        #[clap(short, long, value_parser)]
        skip_plugin_cache: bool,
    },
    /// 返回：插件窗格 ID（格式：plugin_<id>）
    LaunchPlugin {
        #[clap(short, long, value_parser)]
        floating: bool,
        #[clap(short, long, value_parser)]
        in_place: bool,
        /// 关闭被替换窗格而不是挂起（仅在 --in-place 时生效）
        #[clap(
            long,
            value_parser,
            default_value("false"),
            takes_value(false),
            requires("in-place")
        )]
        close_replaced_pane: bool,
        url: Url,
        #[clap(short, long, value_parser)]
        configuration: Option<PluginUserConfiguration>,
        #[clap(short, long, value_parser)]
        skip_plugin_cache: bool,
    },
    RenameSession {
        name: String,
    },
    /// 向一个或多个插件发送数据，若插件未运行则启动
    #[clap(override_usage(
r#"
zellij action pipe [OPTIONS] [--] <PAYLOAD>

* 向指定插件发送数据：

zellij action pipe --plugin file:/path/to/my/plugin.wasm --name my_pipe_name -- my_arbitrary_data

* 发送到所有正在监听的运行中插件：

zellij action pipe --name my_pipe_name -- my_arbitrary_data

* 将数据管道到此命令的 STDIN，并从插件输出到此命令的 STDOUT

tail -f /tmp/my-live-logfile | zellij action pipe --name logs --plugin https://example.com/my-plugin.wasm | wc -l
"#))]
    Pipe {
        /// 管道名称
        #[clap(short, long, value_parser, display_order(1))]
        name: Option<String>,
        /// 通过此管道发送的数据（为空则监听 STDIN）
        payload: Option<String>,

        #[clap(short, long, value_parser, display_order(2))]
        /// 管道参数
        args: Option<PluginUserConfiguration>, // TODO: we might want to not re-use
        // PluginUserConfiguration
        /// 此管道目标插件 URL（如 file:/tmp/my-plugin.wasm）
        /// 若不指定则发送到所有插件；若指定且未运行则启动插件
        #[clap(short, long, value_parser, display_order(3))]
        plugin: Option<String>,
        /// 插件配置（注意：同一插件不同配置会被视为不同插件以决定管道目标）
        #[clap(short('c'), long, value_parser, display_order(4))]
        plugin_configuration: Option<PluginUserConfiguration>,
        /// 即使已有运行中的插件，也强制启动新插件
        #[clap(
            short('l'),
            long,
            value_parser,
            takes_value(false),
            default_value("false"),
            display_order(5)
        )]
        force_launch_plugin: bool,
        /// 启动新插件时跳过缓存并强制重新编译
        #[clap(
            short('s'),
            long,
            value_parser,
            takes_value(false),
            default_value("false"),
            display_order(6)
        )]
        skip_plugin_cache: bool,
        /// 启动插件时是否浮动，默认浮动
        #[clap(short('f'), long, value_parser, display_order(7))]
        floating_plugin: Option<bool>,
        /// 启动插件时是否原位打开（覆盖当前窗格）
        #[clap(
            short('i'),
            long,
            value_parser,
            conflicts_with("floating-plugin"),
            display_order(8)
        )]
        in_place_plugin: Option<bool>,
        /// 启动插件时指定工作目录
        #[clap(short('w'), long, value_parser, display_order(9))]
        plugin_cwd: Option<PathBuf>,
        /// 启动插件时指定窗格标题
        #[clap(short('t'), long, value_parser, display_order(10))]
        plugin_title: Option<String>,
    },
    ListClients,
    /// 列出当前会话中的所有窗格
    ///
    /// 返回：格式化窗格列表（表格或 JSON）到 stdout
    ListPanes {
        /// 包含标签页信息（名称、位置、ID）
        #[clap(short, long, value_parser)]
        tab: bool,

        /// 包含运行命令信息
        #[clap(short, long, value_parser)]
        command: bool,

        /// 包含窗格状态（聚焦、浮动、已退出等）
        #[clap(short, long, value_parser)]
        state: bool,

        /// 包含几何信息（位置、尺寸）
        #[clap(short, long, value_parser)]
        geometry: bool,

        /// 包含所有可用字段
        #[clap(short, long, value_parser)]
        all: bool,

        /// 以 JSON 输出
        #[clap(short, long, value_parser)]
        json: bool,
    },
    /// 列出所有标签页及其信息
    ///
    /// 返回：标签页信息（表格或 JSON 格式）
    ListTabs {
        /// 包含状态信息（激活、全屏、同步、浮动可见性）
        #[clap(short, long, value_parser)]
        state: bool,

        /// 包含尺寸信息（viewport、显示区域）
        #[clap(short, long, value_parser)]
        dimensions: bool,

        /// 包含窗格数量
        #[clap(short, long, value_parser)]
        panes: bool,

        /// 包含布局信息（交换布局名称与 dirty 状态）
        #[clap(short, long, value_parser)]
        layout: bool,

        /// 包含所有可用字段
        #[clap(short, long, value_parser)]
        all: bool,

        /// 以 JSON 输出
        #[clap(short, long, value_parser)]
        json: bool,
    },
    /// 获取当前活动标签页信息
    ///
    /// 返回：默认输出标签页名称和 ID，或以 JSON 输出完整信息
    CurrentTabInfo {
        /// 以 JSON 输出完整 TabInfo
        #[clap(short, long, value_parser)]
        json: bool,
    },
    TogglePanePinned,
    /// 堆叠 pane id
    /// Ids 是以空格分隔的 pane id 列表
    /// 可以是 `terminal_<int>`（如 terminal_1）、`plugin_<int>`（如 plugin_1）
    /// 或裸整数（如 1，等价于 terminal_1）
    ///
    /// 示例：zellij action stack-panes -- terminal_1 plugin_2 3
    StackPanes {
        #[clap(last(true), required(true))]
        pane_ids: Vec<String>,
    },
    ChangeFloatingPaneCoordinates {
        /// 浮动窗格的 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: String,
        /// 浮动窗格 x 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long)]
        x: Option<String>,
        /// 浮动窗格 y 坐标，可填整数（如 1）或百分比（如 10%）
        #[clap(short, long)]
        y: Option<String>,
        /// 浮动窗格宽度，可填整数（如 1）或百分比（如 10%）
        #[clap(long)]
        width: Option<String>,
        /// 浮动窗格高度，可填整数（如 1）或百分比（如 10%）
        #[clap(long)]
        height: Option<String>,
        /// 是否固定浮动窗格于顶层
        #[clap(long)]
        pinned: Option<bool>,
        /// 设置该窗格是否带边框（警告：无边框时无法用鼠标移动）
        #[clap(short, long, value_parser)]
        borderless: Option<bool>,
    },
    TogglePaneBorderless {
        /// 目标 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: String,
    },
    SetPaneBorderless {
        /// 目标 pane_id，如 terminal_1、plugin_2 或 3（等价于 terminal_3）
        #[clap(short, long, value_parser)]
        pane_id: String,
        /// 窗格是否无边框（参数存在）或有边框（参数不存在）
        #[clap(short, long, value_parser)]
        borderless: bool,
    },
    /// 从当前会话分离
    Detach,
    /// 切换到其他会话
    SwitchSession {
        /// 要切换到的会话名称
        name: String,
        /// 可选：要聚焦的标签页位置
        #[clap(long)]
        tab_position: Option<usize>,
        /// 可选：要聚焦的 pane ID（如 "terminal_1" 或 "plugin_2"）
        #[clap(long)]
        pane_id: Option<String>,
        /// 切换会话时应用的布局（相对路径从 layout-dir 起算）
        #[clap(short, long, value_parser)]
        layout: Option<PathBuf>,
        /// 查找布局的默认目录
        #[clap(long, value_parser, requires("layout"))]
        layout_dir: Option<PathBuf>,
        /// 切换时更改工作目录
        #[clap(short, long, value_parser)]
        cwd: Option<PathBuf>,
    },
}
