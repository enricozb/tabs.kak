#define NUM_ARGS 4

struct args {
  int cols;
  char sep;
  int focused_buf_idx;

  int numbufs;
  int *buflens;
  char **buffers;
};

struct args *parse(int argc, char *argv[]) {
  if (argc <= NUM_ARGS + 1) {
    printf("not enough args\n");
    exit(1);
  }

  fprintf(stderr, "args:\n");
  for (int i = 0; i < argc; i++) {
    fprintf(stderr, "%s\n", argv[i]);
  }

  struct args *args = (struct args *)malloc(sizeof(struct args));

  args->cols = atoi(argv[1]) * atoi(argv[2]) / 100;
  args->sep = argv[3][0];
  args->focused_buf_idx = -1;
  args->numbufs = argc - NUM_ARGS - 1;
  args->buflens = (int *)malloc(args->numbufs * sizeof(int));
  args->buffers = (argv + NUM_ARGS + 1);

  char *focused_buf = argv[4];
  for (int i = 0; i < args->numbufs; i++) {
    fprintf(stderr, "buffer: %s\n", args->buffers[i]);
    if (args->focused_buf_idx == -1 &&
        strcmp(args->buffers[i], focused_buf) == 0) {
      args->focused_buf_idx = i;
      fprintf(stderr, "focused_buf_idx=%i\n", i);
    }

    args->buffers[i] = basename(args->buffers[i]);
    args->buflens[i] = strlen(args->buffers[i]);
  }

  return args;
}

char *modelinefmt;
int modelinefmt_len = 0;
int modelinefmt_i = 0;

static inline void init_modelinefmt(int len) {
  modelinefmt = (char *)malloc((len + 1) * sizeof(char));
  modelinefmt[len] = 0;
  modelinefmt_len = len;
}

static inline void write_char(char c) {
  fprintf(stderr, "writing %c\n", c);

  if (modelinefmt_i >= modelinefmt_len) {
    printf("not enough modelinefmt space (%i) \n", modelinefmt_len);
    exit(1);
  }

  modelinefmt[modelinefmt_i] = c;
  modelinefmt_i++;
}

static inline void write_str(char *s) {
  fprintf(stderr, "writing %s\n", s);

  while (*s) {
    write_char(*s);
    s++;
  }
}

static char *focused_buf_format = "{Prompt}";
static int focused_buf_format_len = 8;
static char *other_buf_format = "{LineNumbers}";
static int other_buf_format_len = 13;
static char *sep_format = "{Default}";
static int sep_format_len = 9;
static char *ellipses = "…";
static int ellipses_len = 3;

// tabs_full prints out the modelinefmt assuming we have more space than
// necessary to show all tabs
void tabs_full(int len_total, struct args *args) {
  fprintf(stderr, "tabs_full len_total=%i\n", len_total);

  // focused buffer + other buffers + separator formats
  // separator doesn't have a +1 here because the sepator format is {Default},
  // which the first separator has for free.
  int len_formats = focused_buf_format_len +
                    other_buf_format_len * (args->numbufs - 1) +
                    sep_format_len * (args->numbufs);

  fprintf(stderr, "len_formats=%i\n", len_formats);
  fprintf(stderr, "args->cols=%i\n", args->cols);
  init_modelinefmt(len_total + len_formats);

  // print any leading whitespace
  // int padding = args->cols - len_total;
  // while (padding-- > 0) {
  //   write_char(' ');
  // }

  // create modelinefmt
  write_char('|');
  for (int i = 0; i < args->numbufs; i++) {
    write_char(' ');
    write_str(i == args->focused_buf_idx ? focused_buf_format
                                         : other_buf_format);
    write_str(args->buffers[i]);
    write_str(sep_format);
    write_char(' ');
    write_char('|');
  }

  printf("%s\n", modelinefmt);
}

void tabs_compact(struct args *args) {
  fprintf(stderr, "tabs_compact\n");
  int num_seps = args->numbufs + 1;
  int num_spaces = 2 * args->numbufs;

  // the space available for the names of non-focused buffers.
  int other_bufs_available_space =
      args->cols - num_seps - num_spaces - args->buflens[args->focused_buf_idx];

  // this is the allowed space for the name of each non-focused buffer.
  int space_per_bufs = other_bufs_available_space / (args->numbufs - 1);
  int space_per_bufs_rem = other_bufs_available_space % (args->numbufs - 1);

  // focused buffer + other buffers + separator formats
  // separator doesn't have a +1 here because the sepator format is
  // {Default}, which the first separator has for free.
  int len_formats =
      focused_buf_format_len + other_buf_format_len * (args->numbufs - 1) +
      sep_format_len * (args->numbufs) + ellipses_len * (args->numbufs);

  fprintf(stderr,
          "vars:\n"
          "  available_space=%d\n"
          "  space_per=%d\n"
          "  space_rem=%d\n"
          "  len_formats=%d\n",
          other_bufs_available_space, space_per_bufs, space_per_bufs_rem,
          len_formats);

  init_modelinefmt(args->cols + len_formats);

  // create modelinefmt
  write_char('|');
  for (int i = 0; i < args->numbufs; i++) {
    write_char(' ');
    write_str(i == args->focused_buf_idx ? focused_buf_format
                                         : other_buf_format);
    if (i == args->focused_buf_idx) {
      write_str(args->buffers[i]);
    } else {
      // if the length of this buffers is greater than the allocated space
      // per buffer, cut it off. If there is additional remaining space,
      // increment the cutoff by one.
      bool too_long = args->buflens[i] > space_per_bufs;
      if (too_long && space_per_bufs_rem > 0) {
        args->buffers[i][space_per_bufs] = 0;
        space_per_bufs_rem--;
      } else if (too_long) {
        args->buffers[i][space_per_bufs - 1] = 0;
      }
      write_str(args->buffers[i]);
      if (too_long) {
        write_str(ellipses);
      }
    }

    write_str(sep_format);
    write_char(' ');
    write_char('|');
  }

  // right pad with any remaining space
  // while (space_per_bufs_rem > 0) {
  //   write_char(' ');
  //   space_per_bufs_rem--;
  // }

  printf("%s\n", modelinefmt);
}

void tabs(struct args *args) {
  // length of all buffers
  int len_bufs = 0;
  for (int i = 0; i < args->numbufs; i++) {
    len_bufs += args->buflens[i];
  }

  // length of all separators
  int len_seps = args->numbufs + 1;
  int len_spaces = args->numbufs * 2;

  // total space taken up by tabs
  int len_total = len_bufs + len_seps + len_spaces;

  // if we have more space than we need, don't do any shortening alg
  if (len_total <= args->cols) {
    tabs_full(len_total, args);
  } else {
    tabs_compact(args);
  }
}

// usage:
//   tabs.com  <cols> <percentage> <sep> buf1 buf2 ...
int main(int argc, char *argv[]) {
  struct args *args = parse(argc, argv);
  tabs(args);

  return 0;
}
