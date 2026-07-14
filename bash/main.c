#if defined (NO_MAIN_ENV_ARG)
extern int common_main(int, char**);
int
main(int argc, char **argv){
    return common_main(argc, argv);
}
#else
extern int common_main(int, char**, char**);
int
main(int argc, char **argv, char **env)
{
    return common_main(argc, argv, env);
}
#endif
