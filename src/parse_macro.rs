
#[macro_export]
macro_rules! parse {
    (buf $buf:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::Parse>::parse_buf($buf, false),
            || format!("Parsing {}", $context)
        )
    };
    (buf $buf:expr, $type:ty, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::ParseWithParam<_>>::parse_buf($buf, $param, false),
            || format!("Parsing {}", $context)
        )
    };
    (buf $buf:expr, $type:ty, $context:expr, $trace: expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::Parse>::parse_buf($buf, $trace),
            || format!("Parsing {}", $context)
        )
    };
    (buf $buf:expr, $type:ty, param $param:expr, $context:expr, $trace:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::ParseWithParam<_>>::parse_buf($buf, $param, $trace),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::Parse>::parse($parser),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $type:ty, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::ParseWithParam<_>>::parse($parser, $param),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, bytes $length:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Parser::next($parser, $length),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, data $length:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Data::parse_len($parser, $length),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Parse::parse($parser),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::ParseWithParam::parse($parser, $param),
            || format!("Parsing {}", $context)
        )
    };
}
