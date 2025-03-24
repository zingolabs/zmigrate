#[cfg(not(feature = "with-context"))]
#[macro_export]
macro_rules! parse {
    (buf = $buf:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::Parse>::parse_buf($buf, false),
            format!("Parsing {}", $context),
        )
    };
    (buf = $buf:expr, $type:ty, param = $param:expr, $context:expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse_buf($buf, $param, false),
            format!("Parsing {}", $context),
        )
    };
    (buf = $buf:expr, $type:ty, $context:expr, $trace: expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::Parse>::parse_buf($buf, $trace),
            format!("Parsing {}", $context),
        )
    };
    (buf = $buf:expr, $type:ty, param = $param:expr, $context:expr, $trace:expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse_buf($buf, $param, $trace),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::Parse>::parse($parser),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, $type:ty, param = $param:expr, $context:expr) => {
        ::anyhow::Context::context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse($parser, $param),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, bytes = $length:expr, $context:expr) => {
        ::anyhow::Context::context(
            $crate::zewif::parser::Parser::next($parser, $length),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, data = $length:expr, $context:expr) => {
        ::anyhow::Context::context(
            $crate::zewif::Data::parse_len($parser, $length),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, $context:expr) => {
        ::anyhow::Context::context(
            $crate::zewif::parser::Parse::parse($parser),
            format!("Parsing {}", $context),
        )
    };
    ($parser:expr, param = $param:expr, $context:expr) => {
        ::anyhow::Context::context(
            $crate::zewif::parser::ParseWithParam::parse($parser, $param),
            format!("Parsing {}", $context),
        )
    };
}

#[cfg(feature = "with-context")]
#[macro_export]
macro_rules! parse {
    (buf = $buf:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(<$type as $crate::zewif::parser::Parse>::parse_buf($buf, false), || {
            format!("Parsing {}", $context)
        })
    };
    (buf = $buf:expr, $type:ty, param = $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse_buf($buf, $param, false),
            || format!("Parsing {}", $context),
        )
    };
    (buf = $buf:expr, $type:ty, $context:expr, $trace: expr) => {
        ::anyhow::Context::with_context(<$type as $crate::zewif::parser::Parse>::parse_buf($buf, $trace), || {
            format!("Parsing {}", $context)
        })
    };
    (buf = $buf:expr, $type:ty, param = $param:expr, $context:expr, $trace:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse_buf($buf, $param, $trace),
            || format!("Parsing {}", $context),
        )
    };
    ($parser:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(<$type as $crate::zewif::parser::Parse>::parse($parser), || {
            format!("Parsing {}", $context)
        })
    };
    ($parser:expr, $type:ty, param = $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::zewif::parser::ParseWithParam<_>>::parse($parser, $param),
            || format!("Parsing {}", $context),
        )
    };
    ($parser:expr, bytes = $length:expr, $context:expr) => {
        ::anyhow::Context::with_context($crate::zewif::parser::Parser::next($parser, $length), || {
            format!("Parsing {}", $context)
        })
    };
    ($parser:expr, data = $length:expr, $context:expr) => {
        ::anyhow::Context::with_context($crate::zewif::Data::parse_len($parser, $length), || {
            format!("Parsing {}", $context)
        })
    };
    ($parser:expr, $context:expr) => {
        ::anyhow::Context::with_context($crate::zewif::parser::Parse::parse($parser), || {
            format!("Parsing {}", $context)
        })
    };
    ($parser:expr, param = $param:expr, $context:expr) => {
        ::anyhow::Context::with_context($crate::zewif::parser::ParseWithParam::parse($parser, $param), || {
            format!("Parsing {}", $context)
        })
    };
}
