(function() {var implementors = {};
implementors["allegro"] = [{"text":"impl Send for BitmapFlags","synthetic":true,"types":[]},{"text":"impl Send for Color","synthetic":true,"types":[]},{"text":"impl Send for PixelFormat","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Send for ConfigSection&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Send for ConfigEntry&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl Send for BitmapDrawingFlags","synthetic":true,"types":[]},{"text":"impl Send for BlendMode","synthetic":true,"types":[]},{"text":"impl Send for BlendOperation","synthetic":true,"types":[]},{"text":"impl Send for MonitorInfo","synthetic":true,"types":[]},{"text":"impl Send for DepthFunction","synthetic":true,"types":[]},{"text":"impl Send for Core","synthetic":true,"types":[]},{"text":"impl Send for DisplayFlags","synthetic":true,"types":[]},{"text":"impl Send for DisplayOption","synthetic":true,"types":[]},{"text":"impl Send for DisplayOptionImportance","synthetic":true,"types":[]},{"text":"impl Send for DisplayOrientation","synthetic":true,"types":[]},{"text":"impl !Send for EventQueue","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Send for EventSource&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl Send for UserEventSource","synthetic":true,"types":[]},{"text":"impl !Send for Event","synthetic":true,"types":[]},{"text":"impl Send for StickFlags","synthetic":true,"types":[]},{"text":"impl !Send for Joystick","synthetic":true,"types":[]},{"text":"impl Send for KeyCode","synthetic":true,"types":[]},{"text":"impl Send for KeyModifier","synthetic":true,"types":[]},{"text":"impl Send for ShaderPlatform","synthetic":true,"types":[]},{"text":"impl Send for ShaderType","synthetic":true,"types":[]},{"text":"impl !Send for Timer","synthetic":true,"types":[]},{"text":"impl Send for Bitmap","synthetic":false,"types":[]},{"text":"impl Send for SubBitmap","synthetic":false,"types":[]},{"text":"impl Send for Config","synthetic":false,"types":[]},{"text":"impl Send for Display","synthetic":false,"types":[]},{"text":"impl Send for Shader","synthetic":false,"types":[]},{"text":"impl Send for Transform","synthetic":false,"types":[]}];
implementors["allegro_acodec"] = [{"text":"impl Send for AcodecAddon","synthetic":true,"types":[]}];
implementors["allegro_audio"] = [{"text":"impl Send for AudioAddon","synthetic":true,"types":[]},{"text":"impl Send for AudioDepth","synthetic":true,"types":[]},{"text":"impl Send for ChannelConf","synthetic":true,"types":[]},{"text":"impl Send for Playmode","synthetic":true,"types":[]},{"text":"impl Send for MixerQuality","synthetic":true,"types":[]},{"text":"impl Send for Mixer","synthetic":false,"types":[]},{"text":"impl Send for Sample","synthetic":false,"types":[]},{"text":"impl Send for SampleInstance","synthetic":false,"types":[]},{"text":"impl Send for Sink","synthetic":false,"types":[]},{"text":"impl Send for AudioStream","synthetic":false,"types":[]}];
implementors["allegro_audio_sys"] = [{"text":"impl Send for ALLEGRO_AUDIO_DEPTH","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_CHANNEL_CONF","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_PLAYMODE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_MIXER_QUALITY","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SAMPLE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SAMPLE_ID","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SAMPLE_INSTANCE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_AUDIO_STREAM","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_MIXER","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_VOICE","synthetic":true,"types":[]}];
implementors["allegro_dialog"] = [{"text":"impl Send for MessageBoxFlags","synthetic":true,"types":[]},{"text":"impl Send for MessageBoxResult","synthetic":true,"types":[]},{"text":"impl Send for DialogAddon","synthetic":true,"types":[]}];
implementors["allegro_dialog_sys"] = [{"text":"impl Send for ALLEGRO_FILECHOOSER","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_TEXTLOG","synthetic":true,"types":[]}];
implementors["allegro_font"] = [{"text":"impl Send for FontAddon","synthetic":true,"types":[]},{"text":"impl Send for FontAlign","synthetic":true,"types":[]},{"text":"impl Send for Font","synthetic":false,"types":[]}];
implementors["allegro_font_sys"] = [{"text":"impl !Send for ALLEGRO_FONT","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_FONT_VTABLE","synthetic":true,"types":[]}];
implementors["allegro_image"] = [{"text":"impl Send for ImageAddon","synthetic":true,"types":[]}];
implementors["allegro_primitives"] = [{"text":"impl Send for PrimType","synthetic":true,"types":[]},{"text":"impl Send for LineJoinType","synthetic":true,"types":[]},{"text":"impl Send for LineCapType","synthetic":true,"types":[]},{"text":"impl Send for PrimitivesAddon","synthetic":true,"types":[]},{"text":"impl Send for Vertex","synthetic":true,"types":[]},{"text":"impl Send for VertexDeclBuilder","synthetic":true,"types":[]},{"text":"impl Send for VertexAttrStorage","synthetic":true,"types":[]},{"text":"impl !Send for VertexDecl","synthetic":true,"types":[]}];
implementors["allegro_primitives_sys"] = [{"text":"impl Send for ALLEGRO_VERTEX_ELEMENT","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_VERTEX_DECL","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_VERTEX","synthetic":true,"types":[]}];
implementors["allegro_sys"] = [{"text":"impl Send for ALLEGRO_TIMEOUT","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_BITMAP","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_LOCKED_REGION","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_COLOR","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_CONFIG_SECTION","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_CONFIG_ENTRY","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_CONFIG","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_DISPLAY","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_EVENT_SOURCE","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_ANY_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_DISPLAY_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_JOYSTICK_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_KEYBOARD_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_MOUSE_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_TIMER_EVENT","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_USER_EVENT","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_EVENT","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_EVENT_QUEUE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_FILE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_FILE_INTERFACE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SEEK","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_JOYSTICK","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_JOYSTICK_STATE","synthetic":true,"types":[]},{"text":"impl Send for Stick","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_KEYBOARD","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_KEYBOARD_STATE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_MONITOR_INFO","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_MOUSE","synthetic":true,"types":[]},{"text":"impl !Send for ALLEGRO_MOUSE_STATE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_PATH","synthetic":true,"types":[]},{"text":"impl !Send for __al_tagbstring","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_RENDER_STATE","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_RENDER_FUNCTION","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_WRITE_MASK_FLAGS","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SHADER","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_SYSTEM","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_TIMER","synthetic":true,"types":[]},{"text":"impl Send for ALLEGRO_TRANSFORM","synthetic":true,"types":[]}];
implementors["allegro_ttf"] = [{"text":"impl Send for TtfFlags","synthetic":true,"types":[]},{"text":"impl Send for TtfAddon","synthetic":true,"types":[]}];
implementors["libc"] = [{"text":"impl Send for statvfs","synthetic":true,"types":[]},{"text":"impl Send for max_align_t","synthetic":true,"types":[]},{"text":"impl Send for sigaction","synthetic":true,"types":[]},{"text":"impl Send for statfs","synthetic":true,"types":[]},{"text":"impl Send for flock","synthetic":true,"types":[]},{"text":"impl Send for flock64","synthetic":true,"types":[]},{"text":"impl Send for siginfo_t","synthetic":true,"types":[]},{"text":"impl !Send for stack_t","synthetic":true,"types":[]},{"text":"impl Send for stat","synthetic":true,"types":[]},{"text":"impl Send for stat64","synthetic":true,"types":[]},{"text":"impl Send for statfs64","synthetic":true,"types":[]},{"text":"impl Send for statvfs64","synthetic":true,"types":[]},{"text":"impl Send for pthread_attr_t","synthetic":true,"types":[]},{"text":"impl Send for _libc_fpxreg","synthetic":true,"types":[]},{"text":"impl Send for _libc_xmmreg","synthetic":true,"types":[]},{"text":"impl Send for _libc_fpstate","synthetic":true,"types":[]},{"text":"impl Send for user_regs_struct","synthetic":true,"types":[]},{"text":"impl !Send for user","synthetic":true,"types":[]},{"text":"impl !Send for mcontext_t","synthetic":true,"types":[]},{"text":"impl Send for ipc_perm","synthetic":true,"types":[]},{"text":"impl Send for shmid_ds","synthetic":true,"types":[]},{"text":"impl Send for termios2","synthetic":true,"types":[]},{"text":"impl Send for ip_mreqn","synthetic":true,"types":[]},{"text":"impl Send for user_fpregs_struct","synthetic":true,"types":[]},{"text":"impl !Send for ucontext_t","synthetic":true,"types":[]},{"text":"impl Send for sigset_t","synthetic":true,"types":[]},{"text":"impl Send for sysinfo","synthetic":true,"types":[]},{"text":"impl Send for msqid_ds","synthetic":true,"types":[]},{"text":"impl Send for sem_t","synthetic":true,"types":[]},{"text":"impl Send for statx","synthetic":true,"types":[]},{"text":"impl Send for statx_timestamp","synthetic":true,"types":[]},{"text":"impl !Send for aiocb","synthetic":true,"types":[]},{"text":"impl Send for __exit_status","synthetic":true,"types":[]},{"text":"impl Send for __timeval","synthetic":true,"types":[]},{"text":"impl !Send for glob64_t","synthetic":true,"types":[]},{"text":"impl !Send for msghdr","synthetic":true,"types":[]},{"text":"impl Send for cmsghdr","synthetic":true,"types":[]},{"text":"impl Send for termios","synthetic":true,"types":[]},{"text":"impl Send for mallinfo","synthetic":true,"types":[]},{"text":"impl Send for nlmsghdr","synthetic":true,"types":[]},{"text":"impl Send for nlmsgerr","synthetic":true,"types":[]},{"text":"impl Send for nl_pktinfo","synthetic":true,"types":[]},{"text":"impl Send for nl_mmap_req","synthetic":true,"types":[]},{"text":"impl Send for nl_mmap_hdr","synthetic":true,"types":[]},{"text":"impl Send for nlattr","synthetic":true,"types":[]},{"text":"impl !Send for rtentry","synthetic":true,"types":[]},{"text":"impl Send for timex","synthetic":true,"types":[]},{"text":"impl Send for ntptimeval","synthetic":true,"types":[]},{"text":"impl !Send for regex_t","synthetic":true,"types":[]},{"text":"impl Send for utmpx","synthetic":true,"types":[]},{"text":"impl Send for fpos64_t","synthetic":true,"types":[]},{"text":"impl Send for rlimit64","synthetic":true,"types":[]},{"text":"impl !Send for glob_t","synthetic":true,"types":[]},{"text":"impl !Send for passwd","synthetic":true,"types":[]},{"text":"impl !Send for spwd","synthetic":true,"types":[]},{"text":"impl Send for dqblk","synthetic":true,"types":[]},{"text":"impl Send for signalfd_siginfo","synthetic":true,"types":[]},{"text":"impl Send for itimerspec","synthetic":true,"types":[]},{"text":"impl Send for fsid_t","synthetic":true,"types":[]},{"text":"impl Send for packet_mreq","synthetic":true,"types":[]},{"text":"impl Send for cpu_set_t","synthetic":true,"types":[]},{"text":"impl !Send for if_nameindex","synthetic":true,"types":[]},{"text":"impl Send for msginfo","synthetic":true,"types":[]},{"text":"impl Send for sembuf","synthetic":true,"types":[]},{"text":"impl Send for input_event","synthetic":true,"types":[]},{"text":"impl Send for input_id","synthetic":true,"types":[]},{"text":"impl Send for input_absinfo","synthetic":true,"types":[]},{"text":"impl Send for input_keymap_entry","synthetic":true,"types":[]},{"text":"impl Send for input_mask","synthetic":true,"types":[]},{"text":"impl Send for ff_replay","synthetic":true,"types":[]},{"text":"impl Send for ff_trigger","synthetic":true,"types":[]},{"text":"impl Send for ff_envelope","synthetic":true,"types":[]},{"text":"impl Send for ff_constant_effect","synthetic":true,"types":[]},{"text":"impl Send for ff_ramp_effect","synthetic":true,"types":[]},{"text":"impl Send for ff_condition_effect","synthetic":true,"types":[]},{"text":"impl !Send for ff_periodic_effect","synthetic":true,"types":[]},{"text":"impl Send for ff_rumble_effect","synthetic":true,"types":[]},{"text":"impl Send for ff_effect","synthetic":true,"types":[]},{"text":"impl !Send for dl_phdr_info","synthetic":true,"types":[]},{"text":"impl Send for Elf32_Ehdr","synthetic":true,"types":[]},{"text":"impl Send for Elf64_Ehdr","synthetic":true,"types":[]},{"text":"impl Send for Elf32_Sym","synthetic":true,"types":[]},{"text":"impl Send for Elf64_Sym","synthetic":true,"types":[]},{"text":"impl Send for Elf32_Phdr","synthetic":true,"types":[]},{"text":"impl Send for Elf64_Phdr","synthetic":true,"types":[]},{"text":"impl Send for Elf32_Shdr","synthetic":true,"types":[]},{"text":"impl Send for Elf64_Shdr","synthetic":true,"types":[]},{"text":"impl Send for Elf32_Chdr","synthetic":true,"types":[]},{"text":"impl Send for Elf64_Chdr","synthetic":true,"types":[]},{"text":"impl Send for ucred","synthetic":true,"types":[]},{"text":"impl !Send for mntent","synthetic":true,"types":[]},{"text":"impl !Send for posix_spawn_file_actions_t","synthetic":true,"types":[]},{"text":"impl Send for posix_spawnattr_t","synthetic":true,"types":[]},{"text":"impl Send for genlmsghdr","synthetic":true,"types":[]},{"text":"impl Send for in6_pktinfo","synthetic":true,"types":[]},{"text":"impl Send for arpd_request","synthetic":true,"types":[]},{"text":"impl Send for inotify_event","synthetic":true,"types":[]},{"text":"impl Send for fanotify_response","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_vm","synthetic":true,"types":[]},{"text":"impl Send for regmatch_t","synthetic":true,"types":[]},{"text":"impl Send for sock_extended_err","synthetic":true,"types":[]},{"text":"impl Send for __c_anonymous_sockaddr_can_tp","synthetic":true,"types":[]},{"text":"impl Send for __c_anonymous_sockaddr_can_j1939","synthetic":true,"types":[]},{"text":"impl Send for can_filter","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_nl","synthetic":true,"types":[]},{"text":"impl Send for dirent","synthetic":true,"types":[]},{"text":"impl Send for dirent64","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_alg","synthetic":true,"types":[]},{"text":"impl Send for af_alg_iv","synthetic":true,"types":[]},{"text":"impl Send for mq_attr","synthetic":true,"types":[]},{"text":"impl Send for __c_anonymous_sockaddr_can_can_addr","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_can","synthetic":true,"types":[]},{"text":"impl Send for pthread_mutexattr_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_rwlockattr_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_condattr_t","synthetic":true,"types":[]},{"text":"impl Send for fanotify_event_metadata","synthetic":true,"types":[]},{"text":"impl Send for pthread_cond_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_mutex_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_rwlock_t","synthetic":true,"types":[]},{"text":"impl Send for can_frame","synthetic":true,"types":[]},{"text":"impl Send for canfd_frame","synthetic":true,"types":[]},{"text":"impl Send for timezone","synthetic":true,"types":[]},{"text":"impl Send for in_addr","synthetic":true,"types":[]},{"text":"impl Send for ip_mreq","synthetic":true,"types":[]},{"text":"impl Send for ip_mreq_source","synthetic":true,"types":[]},{"text":"impl Send for sockaddr","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_in","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_in6","synthetic":true,"types":[]},{"text":"impl !Send for addrinfo","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_ll","synthetic":true,"types":[]},{"text":"impl Send for fd_set","synthetic":true,"types":[]},{"text":"impl !Send for tm","synthetic":true,"types":[]},{"text":"impl Send for sched_param","synthetic":true,"types":[]},{"text":"impl !Send for Dl_info","synthetic":true,"types":[]},{"text":"impl !Send for lconv","synthetic":true,"types":[]},{"text":"impl Send for in_pktinfo","synthetic":true,"types":[]},{"text":"impl !Send for ifaddrs","synthetic":true,"types":[]},{"text":"impl Send for in6_rtmsg","synthetic":true,"types":[]},{"text":"impl Send for arpreq","synthetic":true,"types":[]},{"text":"impl Send for arpreq_old","synthetic":true,"types":[]},{"text":"impl Send for arphdr","synthetic":true,"types":[]},{"text":"impl !Send for mmsghdr","synthetic":true,"types":[]},{"text":"impl Send for epoll_event","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_un","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_storage","synthetic":true,"types":[]},{"text":"impl Send for utsname","synthetic":true,"types":[]},{"text":"impl !Send for sigevent","synthetic":true,"types":[]},{"text":"impl Send for in6_addr","synthetic":true,"types":[]},{"text":"impl Send for DIR","synthetic":true,"types":[]},{"text":"impl !Send for group","synthetic":true,"types":[]},{"text":"impl Send for utimbuf","synthetic":true,"types":[]},{"text":"impl Send for timeval","synthetic":true,"types":[]},{"text":"impl Send for timespec","synthetic":true,"types":[]},{"text":"impl Send for rlimit","synthetic":true,"types":[]},{"text":"impl Send for rusage","synthetic":true,"types":[]},{"text":"impl Send for ipv6_mreq","synthetic":true,"types":[]},{"text":"impl !Send for hostent","synthetic":true,"types":[]},{"text":"impl !Send for iovec","synthetic":true,"types":[]},{"text":"impl Send for pollfd","synthetic":true,"types":[]},{"text":"impl Send for winsize","synthetic":true,"types":[]},{"text":"impl Send for linger","synthetic":true,"types":[]},{"text":"impl !Send for sigval","synthetic":true,"types":[]},{"text":"impl Send for itimerval","synthetic":true,"types":[]},{"text":"impl Send for tms","synthetic":true,"types":[]},{"text":"impl !Send for servent","synthetic":true,"types":[]},{"text":"impl !Send for protoent","synthetic":true,"types":[]},{"text":"impl Send for FILE","synthetic":true,"types":[]},{"text":"impl Send for fpos_t","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()