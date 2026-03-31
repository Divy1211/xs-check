# XS Type Checking

## 1. Notation

- $\Gamma$ is a type environment mapping XS identifiers to types.
- $\Gamma \vdash E : T$ means that an expression $E$ has type $T$ in $\Gamma$ (read as $\Gamma$ yields $E$ of type $T$)
- $\Gamma \vdash S$ means that a statement $S$ is soundly typed in $\Gamma$ (read as $\Gamma$ yields $S$)
- $$\begin{array}{rc}
    {\tt (xsTcCase)} & \begin{array}{c}
        \begin{array}{cc} C_1 & C_2 \end{array}
        \\ \hline
        S_1
    \end{array}
\end{array}
$$

    is read as $C_1 \land C_2 \implies S_1$

### 1.1 Well Typed Programs

An XS program $X := \bar{S}$ (a sequence of statements) is said to be well typed iff $\exists\ \Gamma\ |\ \Gamma \vdash X$ (there exists some type environment such that it can type check the program)

## 2. Type Checking For Expressions

### 2.1. Literals

let $L$ denote a literal

$$
\begin{array}{rc}
    {\tt (xsTcInt)} & \begin{array}{c}
        L\ \in \{x | -999,999,999 \leq x \leq 999,999,999 \land x \in \mathbb{Z} \}
        \\ \hline
        \Gamma \vdash L : {\tt int}
    \end{array}
\end{array}
$$

(XS ${\tt int}$ literals may not be more than 9 digits long. yES)

$$
\begin{array}{rc}
    {\tt (xsTcStr)} & \begin{array}{c}
        L\ {\tt is\ a\ "\ delimited\ sequence\ of\ characters}
        \\ \hline
        \Gamma \vdash L : {\tt string}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcFloat)} & \begin{array}{c}
        L\ \in \mathbb{R}
        \\ \hline
        \Gamma \vdash L : {\tt float}
    \end{array}
\end{array}
$$


$$
\begin{array}{rc}
    {\tt (xsTcBool)} & \begin{array}{c}
        L\ \in \{{\tt true,\ false}\}
        \\ \hline
        \Gamma \vdash L : {\tt bool}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcVec)} & \begin{array}{c}
        \begin{array}{cccc}
            L\ |\ L := {\tt vector}(x, y, z)
            & \Gamma \vdash x : {\tt int}\ |\ {\tt float}
            & \Gamma \vdash y : {\tt int}\ |\ {\tt float}
            & \Gamma \vdash z : {\tt int}\ |\ {\tt float}
        \end{array}
        \\ \hline
        \Gamma \vdash L : {\tt vector}
    \end{array}
\end{array}
$$

Note: $x$, $y$, and $z$ must also be literals

### 2.2. Identifier

let $X$ be an identifier

$$
\begin{array}{rc}
    {\tt (xsTcId)} & \begin{array}{c}
        (X, T)\ \in \Gamma
        \\ \hline
        \Gamma \vdash X : T
    \end{array}
\end{array}
$$

### 2.3. Parenthesis

$$
\begin{array}{rc}
    {\tt (xsTcParen)} & \begin{array}{c}
        \Gamma \vdash E : T 
        \\ \hline
        \Gamma \vdash (E) : T
    \end{array}
\end{array}
$$

### 2.4. Function Call (Expression)

$$
\begin{array}{rc}
    {\tt (xsTcFncExpr)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash {\tt fnName} : (T_1, ..., T_n) \rightarrow T_r
            & \Gamma \vdash E_i : T_i\ |\ {\tt void}
            & n \leq 12
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt fnName(E_1, ..., E_n)} : T_r
    \end{array}
\end{array}
$$

Note: XS can have functions of arity $\in [0, 12]$ but every function must define default values for each parameter, which means it is possible to omit any number of its arguments which are initialised with defaults in that case. Hence the ${\tt T_n | void}$ type for parameters. There is no currying in XS :(

### 2.5. Operations

$$
\begin{array}{rc}
    {\tt (xsTcArithInt)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt int}
            & \Gamma \vdash E_2 : {\tt int}\ |\ {\tt float}
            & {\tt op}\ \in \{{\tt +,\ -,\ *,\ /,\ \%}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt int}
    \end{array}
\end{array}
$$

(an ${\tt int op float}$ is an ${\tt int}$ in XS... yES.)

$$
\begin{array}{rc}
    {\tt (xsTcArithFloat)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt float}
            & \Gamma \vdash E_2 : {\tt int}\ |\ {\tt float}
            & {\tt op}\ \in \{{\tt +,\ -,\ *,\ /,\ \%}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt float}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcStrConc1)} & \begin{array}{c}
        \begin{array}{cc}
            \Gamma \vdash E_1 : {\tt string}
            & \Gamma \vdash E_2 : {\tt int}\ |\ {\tt float}\ |\ {\tt bool}\ |\ {\tt string}\ |\ {\tt vector}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt +}\ E_2 : {\tt string}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcStrConc2)} & \begin{array}{c}
        \begin{array}{cc}
            \Gamma \vdash E_1 : {\tt int}\ |\ {\tt float}\ |\ {\tt bool}\ |\ {\tt string}
            & \Gamma \vdash E_2 : {\tt string}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt +}\ E_2 : {\tt string}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcRelnNum)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt int}\ |\ {\tt float}
            & \Gamma \vdash E_2 : {\tt int}\ |\ {\tt float}
            & {\tt op}\ \in \{{\tt <,\ <=,\ >,\ >=,\ ==,\ !=}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt bool}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsTcRelnStr)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt string}
            & \Gamma \vdash E_2 : {\tt string}
            & {\tt op}\ \in \{{\tt <,\ <=,\ >,\ >=,\ ==,\ !=}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt bool}
    \end{array}
\end{array}
$$

Note: strings are compared lexically

$$
\begin{array}{rc}
    {\tt (xsTcEqVec)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt vector}
            & \Gamma \vdash E_2 : {\tt vector}
            & {\tt op}\ \in \{{\tt ==,\ !=}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt bool}
    \end{array}
\end{array}
$$


$$
\begin{array}{rc}
    {\tt (xsTcEqBool)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt bool}
            & \Gamma \vdash E_2 : {\tt bool}
            & {\tt op}\ \in \{{\tt ==,\ !=}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt bool}
    \end{array}
\end{array}
$$

Note: Trying relational operators on vectors and booleans passes the in game type checker, but will cause a silent XS crash.

$$
\begin{array}{rc}
    {\tt (xsTcLogical)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_1 : {\tt bool}
            & \Gamma \vdash E_2 : {\tt bool}
            & {\tt op}\ \in \{{\tt \&\&,\ ||}\}
        \end{array}
        \\ \hline
        \Gamma \vdash E_1\ {\tt op}\ E_2 : {\tt bool}
    \end{array}
\end{array}
$$

## 3. Type Checking For Statements

### 3.1. Sequence

$$
\begin{array}{rc}
    {\tt (xsTcSeq)} & \begin{array}{c}
        \begin{array}{cc}
            \Gamma \vdash S
            & \Gamma \vdash \bar{S}
        \end{array}
        \\ \hline
        \Gamma \vdash S\bar{S}
    \end{array}
\end{array}
$$

### 3.2. Include

$$
\begin{array}{rc}
    {\tt (xsTcInc)} & \begin{array}{c}
        \Gamma \vdash S : {\tt string}
        \\ \hline
        \Gamma \vdash {\tt include\ }S{\tt;}
    \end{array}
\end{array}
$$

### 3.3. Var Def

$$
\begin{array}{rc}
    {\tt (xsTcDef)} & \begin{array}{c}
        \Gamma \vdash E : T
        \\ \hline
        \begin{array}{cc}
            \Gamma \vdash T\ X\ =\ E{\tt ;}
            & \Gamma \oplus (X, T)
        \end{array}
    \end{array}
\end{array}
$$

Note an initialiser in a definition is optional

### 3.4. Var Assign

$$
\begin{array}{rc}
    {\tt (xsTcAssign)} & \begin{array}{c}
        \begin{array}{cc}
            \Gamma \vdash X : T
            & \Gamma \vdash E : T
        \end{array}
        \\ \hline
        \Gamma \vdash X\ =\ E{\tt ;}
    \end{array}
\end{array}
$$

Note: floats may be assigned to ints - they follow the expected casting rules.

### 3.5. If Else

$$
\begin{array}{rc}
    {\tt (xsTcIfElse)} & \begin{array}{c}
        \begin{array}{ccc}
            \Gamma \vdash E_c : {\tt bool}
            & \Gamma \vdash \bar{S_1}
            & \Gamma \vdash \bar{S_2}
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt if\ (} E_c {\tt)\ \{\ } \bar{S_1} {\tt\ \}\ else\ \{\ } \bar{S_2} {\tt\ \}}
    \end{array}
\end{array}
$$

### 3.6. While

$$
\begin{array}{rc}
    {\tt (xsTcWhile)} & \begin{array}{c}
        \begin{array}{cc}
            \Gamma \vdash E_c : {\tt bool}
            & \Gamma \vdash \bar{S}
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}
    \end{array}
\end{array}
$$

### 3.7. For

$$
\begin{array}{rc}
    {\tt (xsTcFor)} & \begin{array}{c}
        \begin{array}{cccc}
            \Gamma \vdash E_1 : {\tt int}\ |\ {\tt float}\ |\ {\tt bool}
            & \Gamma \vdash E_2 : {\tt int}\ |\ {\tt float}
            & \Gamma \vdash \bar{S}
            & \text{OP } \in {\{\tt <,\ <=,\ >,\ >=}\}
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}
    \end{array}
\end{array}
$$

Note: Floats may be used in $E_1$ or $E_2$ they will be cast to int before being used by the loop. Bools in the initializer are cast to int, bools in the conditional pass the in game type checker but will cause a silent XS crash.


### 3.8. Switch

$$
\begin{array}{rc}
    {\tt (xsTcSwitch)} & \begin{array}{c}
        \begin{array}{cccc}
            \Gamma \vdash E_c : {\tt int}\ |\ {\tt float}\ |\ {\tt bool}
            & \Gamma \vdash E_i : {\tt int}\ |\ {\tt float}
            & \Gamma \vdash \bar{S_i}
            & \Gamma \vdash \bar{S_d}
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt switch\ (} E_c {\tt)\ \{\ } {\tt case\ } E_1 {\tt\ :\ \{\ } \bar{S_1} {\tt\ \}} {\tt\ ...\ case\ } E_n {\tt\ :\ \{\ } \bar{S_n} {\tt\ \}} {\tt\ default\ :\ \{\ } \bar{S_d} {\tt\ \}} {\tt\ \}}
    \end{array}
\end{array}
$$

Note: Floats may be used in $E_c$ or $E_n$ they will be cast to int before being used by the cases. Bools in the expression are cast to int, bools in a case's expression pass the in game type checker but will cause a silent XS crash.

### 3.9. Break, Continue, Break Point, Debug

$$\begin{matrix} {\tt (xsTcBr)} & \Gamma \vdash {\tt break;} \end{matrix}$$

$$\begin{matrix} {\tt (xsTcCo)} & \Gamma \vdash {\tt continue;} \end{matrix}$$

$$\begin{matrix} {\tt (xsTcBrPt)} & \Gamma \vdash {\tt breakpoint;} \end{matrix}$$

$$
\begin{array}{rc}
    {\tt (xsTcDbg)} & \begin{array}{c}
        \begin{array}{c}
            \Gamma \vdash X : T
            & T \notin \{A \rightarrow B, {\tt rule}, {\tt label}, {\tt class}\}
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt dbg\ } X{\tt ;}
    \end{array}
\end{array}
$$

### 3.10. Function Definitions

$$
\begin{array}{rc}
    {\tt (xsTcFn)} & \begin{array}{c}
        \begin{array}{cccccc}
            T_r \in \{{\tt int,\ float,\ bool,\ string,\ vector,\ void}\}
            & \Gamma \vdash E_i : T_i
            & n \leq 12
            & \Gamma \vdash \bar{S}
            & {\tt return\ E_r;} \in \bar{S}
            & \Gamma \vdash E_r : T_r
        \end{array}
        \\ \hline
        \begin{array}{cc}
            \Gamma \vdash T_r\ {\tt fnName(T_1\ id_1\ =\ E_1,\ ...,\ T_n\ id_n\ =\ E_n )\ \{\ } \bar{S} {\tt\ \}}
            & \Gamma \oplus ({\tt fnName}, (T_1, ..., T_n) \rightarrow T_r)
        \end{array}
    \end{array}
\end{array}
$$

If the return type of a function is ${\tt void}$, the return statement may be omitted

### 3.11. Rule Definitions

$$
\begin{array}{rc}
    {\tt (xsTcRule)} & \begin{array}{c}
        \Gamma \vdash \bar{S}
        \\ \hline
        \begin{array}{cc}
            \Gamma \vdash {\tt rule\ ruleName\ ruleOpts\ \ \{\ } \bar{S} {\tt\ \}}
            & \Gamma \oplus ({\tt ruleName}, {\tt rule})
        \end{array}
    \end{array}
\end{array}
$$

### 3.12. Postfix

$$
\begin{array}{rc}
    {\tt (xsTcPost)} & \begin{array}{c}
        \begin{array}{cc}
            ({\tt id,\ int\ |\ float}) \in \Gamma
            & \text{##}\ \in \{{\tt ++,\ --}\}
        \end{array}
        \\ \hline
        \Gamma \vdash \text{id##;}
    \end{array}
\end{array}
$$

Postfixes are ~~expressions~~ statements in XS. yES

### 3.13. Label, Goto

$$\begin{matrix} {\tt (xsTcLabel)} & \Gamma \vdash {\tt label\ id;} \end{matrix}$$

$$\begin{matrix} {\tt (xsTcGoto)} & \Gamma \vdash {\tt goto\ id;} \end{matrix}$$

### 3.14. Function Call (Statement)

$({\tt xsTcFncStmt})$ same as [2.4. Function Call (Expression)](#24-function-call-expression) with a terminating semicolon.

### 3.15. Class Definition

$$
\begin{array}{rc}
    {\tt (xsTcClsDef)} & \begin{array}{c}
        \begin{array}{c}
            \Gamma \vdash E_i : T_i
        \end{array}
        \\ \hline
        \Gamma \vdash {\tt class\ clsName\ \{}\ T_1\ id_1\ =\ E_1;\ ...\ T_n\ id_n\ =\ E_n; {\tt\ \};}
    \end{array}
\end{array}
$$

Classes are unused in XS and can't be instantiated afaik. This exists purely for completeness' sake.